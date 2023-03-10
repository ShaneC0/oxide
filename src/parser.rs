use crate::{lexer::Lexer, Token};
use std::{error::Error, fmt, ops::Add};

#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

// expected token got other token
// Expected category of token
// unrecognized token
// unexpected end of input

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        match self.lexer.next() {
            Some(t) => Ok(t),
            None => Err(ParseError {
                msg: "Unexpected end of input.".to_string(),
            }),
        }
    }

    fn cmp_next_token(&mut self, target: Token) -> Result<Token, ParseError> {
        let token = self.next_token()?;
        if std::mem::discriminant(&token) == std::mem::discriminant(&target) {
            return Ok(token);
        }
        if let Token::ERROR(msg) = token {
            Err(ParseError { msg })
        } else {
            self.lexer.push_back(token);
            Err(ParseError {
                msg: format!("Expected token {:?}.", target),
            })
        }
    }

    pub fn cmp_next_token_many(
        &mut self,
        targets: Vec<Token>,
        stmt_type: &str,
    ) -> Result<Token, ParseError> {
        let token = self.next_token()?;
        for target in targets {
            if std::mem::discriminant(&token) == std::mem::discriminant(&target) {
                return Ok(token);
            }
        }
        self.lexer.push_back(token);
        Err(ParseError {
            msg: format!("Expected {}.", stmt_type),
        })
    }

    // Program ::= INIT <StmtList> HALT
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        self.cmp_next_token(Token::INIT)?;
        let stmt_list = self.parse_stmt_list()?;
        self.cmp_next_token(Token::HALT)?;
        Ok(Program { stmt_list })
    }

    // Maybe check for stmtlist ending items here like endwhile endif and halt
    // StmtList ::= <Stmt> SEMICOL { <Stmt> SEMICOL }
    fn parse_stmt_list(&mut self) -> Result<StmtList, ParseError> {
        let mut stmts: Vec<Stmt> = vec![];
        let stmt = self.parse_stmt()?;
        self.cmp_next_token(Token::SEMICOL)?;
        stmts.push(stmt);

        loop {
            match self.cmp_next_token_many(
                vec![Token::HALT, Token::ENDIF, Token::ENDWHILE],
                "ending delimiter",
            ) {
                Ok(t) => {
                    self.lexer.push_back(t);
                    break;
                }
                Err(_) => {
                    let next_stmt = self.parse_stmt()?;
                    self.cmp_next_token(Token::SEMICOL)?;
                    stmts.push(next_stmt);
                }
            }
        }

        Ok(StmtList { stmts })
    }

    // Stmt ::= <DeclStmt> | <CtrlStmt>
    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let token = self.next_token()?;
        let stmt = match token {
            Token::INT | Token::FLOAT | Token::BOOL | Token::STRING => {
                self.lexer.push_back(token);
                let decl_stmt = self.parse_decl_stmt()?;
                Stmt::Decl(decl_stmt)
            }
            _ => {
                self.lexer.push_back(token);
                let ctrl_stmt = self.parse_ctrl_stmt()?;
                Stmt::Ctrl(ctrl_stmt)
            }
        };
        Ok(stmt)
    }

    // DeclStmt ::= (INT | FLOAT | BOOL | STRING) IDENT { COMMA IDENT }
    fn parse_decl_stmt(&mut self) -> Result<DeclStmt, ParseError> {
        let mut idents: Vec<Token> = vec![];
        let type_specifier = self.cmp_next_token_many(
            vec![Token::INT, Token::FLOAT, Token::BOOL, Token::STRING],
            "type specifier",
        )?;
        let token = self.cmp_next_token(Token::IDENT("".to_string()))?;
        idents.push(token);
        while let Ok(_) = self.cmp_next_token(Token::COMMA) {
            let token = self.cmp_next_token(Token::IDENT("".to_string()))?;
            idents.push(token);
        }
        Ok(DeclStmt {
            type_specifier,
            idents,
        })
    }

    // CtrlStmt ::= <AssignStmt> | <PrintStmt> | <IfStmt> | <LoopStmt>
    fn parse_ctrl_stmt(&mut self) -> Result<CtrlStmt, ParseError> {
        let token = self.next_token()?;
        let stmt = match token {
            Token::IDENT(s) => {
                self.lexer.push_back(Token::IDENT(s));
                let assign_stmt = self.parse_assign_stmt()?;
                CtrlStmt::Assign(assign_stmt)
            }
            Token::PRINT => {
                self.lexer.push_back(token);
                let print_stmt = self.parse_print_stmt()?;
                CtrlStmt::Print(print_stmt)
            }
            Token::IF => {
                self.lexer.push_back(token);
                let if_stmt = self.parse_if_stmt()?;
                CtrlStmt::If(if_stmt)
            }
            Token::WHILE => {
                self.lexer.push_back(token);
                let loop_stmt = self.parse_loop_stmt()?;
                CtrlStmt::Loop(loop_stmt)
            }
            _ => {
                self.lexer.push_back(token);
                return Err(ParseError {
                    msg: "Expected control statement".to_string(),
                });
            }
        };
        Ok(stmt)
    }

    // AssignStmt ::= IDENT ASSOP <OrExpr>
    fn parse_assign_stmt(&mut self) -> Result<AssignStmt, ParseError> {
        let ident = self.cmp_next_token(Token::IDENT("".to_string()))?;
        self.cmp_next_token(Token::ASSOP)?;
        let expr = self.parse_or_expr()?;
        Ok(AssignStmt { ident, expr })
    }

    // PrintStmt ::= PRINT OPENPAREN <OrExpr> { COMMA <OrExpr> } CLOSEPAREN
    fn parse_print_stmt(&mut self) -> Result<PrintStmt, ParseError> {
        let mut exprs: Vec<OrExpr> = vec![];
        self.cmp_next_token(Token::PRINT)?;
        self.cmp_next_token(Token::OPENPAREN)?;
        let expr = self.parse_or_expr()?;
        exprs.push(expr);
        while let Ok(_) = self.cmp_next_token(Token::COMMA) {
            let expr = self.parse_or_expr()?;
            exprs.push(expr);
        }
        self.cmp_next_token(Token::CLOSEPAREN)?;
        Ok(PrintStmt { exprs })
    }

    // Contains odd err handling.
    // IfStmt ::= IF OPENPAREN <OrExpr> CLOSEPAREN THEN <StmtList> [ ELSE <StmtList> ] ENDIF
    fn parse_if_stmt(&mut self) -> Result<IfStmt, ParseError> {
        self.cmp_next_token(Token::IF)?;
        self.cmp_next_token(Token::OPENPAREN)?;
        let condition = self.parse_or_expr()?;
        self.cmp_next_token(Token::CLOSEPAREN)?;
        self.cmp_next_token(Token::THEN)?;
        let then_stmts = self.parse_stmt_list()?;
        let token = self.next_token()?;
        let mut else_present = false;
        let else_stmts = match token {
            Token::ELSE => {
                else_present = true;
                Some(self.parse_stmt_list()?)
            }
            Token::ENDIF => None,
            _ => {
                return Err(ParseError {
                    msg: "Expected else or endif.".to_string(),
                })
            }
        };
        if else_present {
            self.cmp_next_token(Token::ENDIF)?;
        }
        Ok(IfStmt {
            condition,
            then_stmts,
            else_stmts,
        })
    }

    // LoopStmt ::= WHILE OPENPAREN <OrExpr> CLOSEPAREN DO <StmtList> ENDWHILE
    fn parse_loop_stmt(&mut self) -> Result<LoopStmt, ParseError> {
        self.cmp_next_token(Token::WHILE)?;
        self.cmp_next_token(Token::OPENPAREN)?;
        let condition = self.parse_or_expr()?;
        self.cmp_next_token(Token::CLOSEPAREN)?;
        self.cmp_next_token(Token::DO)?;
        let stmts = self.parse_stmt_list()?;
        self.cmp_next_token(Token::ENDWHILE)?;
        Ok(LoopStmt { condition, stmts })
    }

    // OrExpr ::= <AndExpr> { OROP <AndExpr> }
    fn parse_or_expr(&mut self) -> Result<OrExpr, ParseError> {
        let mut rhs: Vec<AndExpr> = vec![];
        let lhs = self.parse_and_expr()?;
        while let Ok(_) = self.cmp_next_token(Token::OROP) {
            let expr = self.parse_and_expr()?;
            rhs.push(expr)
        }
        Ok(OrExpr { lhs, rhs })
    }

    // AndExpr ::= <EqualExpr> { ANDOP <EqualExpr> }
    fn parse_and_expr(&mut self) -> Result<AndExpr, ParseError> {
        let mut rhs: Vec<EqualExpr> = vec![];
        let lhs = self.parse_equal_expr()?;
        while let Ok(_) = self.cmp_next_token(Token::ANDOP) {
            let expr = self.parse_equal_expr()?;
            rhs.push(expr);
        }
        Ok(AndExpr { lhs, rhs })
    }

    // EqualExpr ::= <RelExpr> [ EQUALOP <RelExpr> ]
    fn parse_equal_expr(&mut self) -> Result<EqualExpr, ParseError> {
        let lhs = self.parse_rel_expr()?;
        let mut rhs: Option<RelExpr> = None;
        if let Ok(_) = self.cmp_next_token(Token::EQUALOP) {
            rhs = Some(self.parse_rel_expr()?);
        }
        Ok(EqualExpr { lhs, rhs })
    }

    // More odd error handling
    // RelExpr ::= <AddExpr> [ (LTHAN | GTHAN) <AddExpr> ]
    fn parse_rel_expr(&mut self) -> Result<RelExpr, ParseError> {
        let lhs = self.parse_add_expr()?;
        let op = self
            .cmp_next_token_many(vec![Token::LTHAN, Token::GTHAN], "relational operator")
            .ok();
        let rhs = match op {
            Some(_) => Some(self.parse_add_expr()?),
            None => None,
        };
        Ok(RelExpr { lhs, op, rhs })
    }

    // AddExpr ::= <MultExpr> { (PLUS | MINUS) <MultExpr> }
    fn parse_add_expr(&mut self) -> Result<AddExpr, ParseError> {
        let lhs = self.parse_mult_expr()?;
        let mut ops: Vec<Token> = vec![];
        let mut rhs: Vec<MultExpr> = vec![];
        while let Ok(op) =
            self.cmp_next_token_many(vec![Token::PLUS, Token::MINUS], "addition operator")
        {
            ops.push(op);
            let expr = self.parse_mult_expr()?;
            rhs.push(expr);
        }
        Ok(AddExpr { lhs, ops, rhs })
    }

    // MultExpr ::= <UnaryExpr> { (MULT | DIV | MOD) <UnaryExpr> }
    fn parse_mult_expr(&mut self) -> Result<MultExpr, ParseError> {
        let lhs = self.parse_unary_expr()?;
        let mut ops: Vec<Token> = vec![];
        let mut rhs: Vec<UnaryExpr> = vec![];
        while let Ok(op) = self.cmp_next_token_many(
            vec![Token::MULT, Token::DIV, Token::MOD],
            "multiplication operator",
        ) {
            ops.push(op);
            let expr = self.parse_unary_expr()?;
            rhs.push(expr);
        }
        Ok(MultExpr { lhs, ops, rhs })
    }

    // UnaryExpr ::= [ (NOT | MINUS) ] <PrimaryExpr>
    fn parse_unary_expr(&mut self) -> Result<UnaryExpr, ParseError> {
        let op = self
            .cmp_next_token_many(vec![Token::NOT, Token::MINUS], "unary operator")
            .ok();
        let expr = self.parse_primary_expr()?;
        Ok(UnaryExpr { op, expr })
    }

    // The ( <OrExpr> ) is required to use parenthesis to control order of operations.
    // PrimaryExpr ::= IDENT | ICONST | FCONST | BCONST | SCONST
    fn parse_primary_expr(&mut self) -> Result<PrimaryExpr, ParseError> {
        let constant = self.cmp_next_token_many(
            vec![
                Token::IDENT("".to_string()),
                Token::ICONST(0),
                Token::FCONST(0.0),
                Token::BCONST(false),
                Token::SCONST("".to_string()),
            ],
            "literal",
        )?;
        Ok(PrimaryExpr { constant })
    }
}

#[derive(Debug)]
pub struct Program {
    stmt_list: StmtList,
}
#[derive(Debug)]
pub struct StmtList {
    stmts: Vec<Stmt>,
}
#[derive(Debug)]
pub enum Stmt {
    Decl(DeclStmt),
    Ctrl(CtrlStmt),
}
#[derive(Debug)]
pub struct DeclStmt {
    type_specifier: Token,
    idents: Vec<Token>,
}
#[derive(Debug)]
pub enum CtrlStmt {
    Assign(AssignStmt),
    Print(PrintStmt),
    If(IfStmt),
    Loop(LoopStmt),
}
#[derive(Debug)]
pub struct AssignStmt {
    ident: Token,
    expr: OrExpr,
}
#[derive(Debug)]
pub struct PrintStmt {
    exprs: Vec<OrExpr>,
}
#[derive(Debug)]
pub struct IfStmt {
    condition: OrExpr,
    then_stmts: StmtList,
    else_stmts: Option<StmtList>,
}
#[derive(Debug)]
pub struct LoopStmt {
    condition: OrExpr,
    stmts: StmtList,
}
#[derive(Debug)]
pub struct OrExpr {
    lhs: AndExpr,
    rhs: Vec<AndExpr>,
}
#[derive(Debug)]
pub struct AndExpr {
    lhs: EqualExpr,
    rhs: Vec<EqualExpr>,
}
#[derive(Debug)]
pub struct EqualExpr {
    lhs: RelExpr,
    rhs: Option<RelExpr>,
}
#[derive(Debug)]
pub struct RelExpr {
    lhs: AddExpr,
    op: Option<Token>,
    rhs: Option<AddExpr>,
}
#[derive(Debug)]
pub struct AddExpr {
    lhs: MultExpr,
    ops: Vec<Token>,
    rhs: Vec<MultExpr>,
}
#[derive(Debug)]
pub struct MultExpr {
    lhs: UnaryExpr,
    ops: Vec<Token>,
    rhs: Vec<UnaryExpr>,
}
#[derive(Debug)]
pub struct UnaryExpr {
    op: Option<Token>,
    expr: PrimaryExpr,
}
#[derive(Debug)]
pub struct PrimaryExpr {
    constant: Token,
}
