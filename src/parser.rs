use crate::{lexer::Lexer, Token};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.msg)
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
    fn cmp_next_token(&mut self, target: Token) -> Result<Token, ParseError> {
        let token = match self.lexer.next() {
            Some(t) => t,
            None => {
                return Err(ParseError {
                    msg: "Unexpected end of input.".to_string(),
                })
            }
        };

        if std::mem::discriminant(&token) == std::mem::discriminant(&target) {
            return Ok(token);
        }

        match token {
            Token::ERROR(msg) => Err(ParseError { msg }),
            _ => {
                self.lexer.push_back(token);
                Err(ParseError {
                    // How do i make it display the target token variant?
                    msg: "Expected token {placeholder)".to_string(),
                })
            }
        }
    }

    // Program ::= INIT <StmtList> HALT
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        if let Err(e) = self.cmp_next_token(Token::INIT) {
            return Err(e);
        }

        let stmt_list = match self.parse_stmt_list() {
            Ok(stmts) => stmts,
            Err(e) => return Err(e),
        };

        if let Err(e) = self.cmp_next_token(Token::HALT) {
            return Err(e);
        }

        Ok(Program { stmt_list })
    }

    // StmtList ::= <Stmt>; { <Stmt>; }
    fn parse_stmt_list(&mut self) -> Result<StmtList, ParseError> {
        let mut stmts: Vec<Stmt> = vec![];
        let stmt = match self.parse_stmt() {
            Ok(_stmt) => _stmt,
            Err(parse_error) => return Err(parse_error),
        };

        if let Err(parse_error) = self.cmp_next_token(Token::SEMICOL) {
            return Err(parse_error);
        }

        stmts.push(stmt);

        // This ignores when an error is returned.
        // Not sure if this is the right way to do this.
        // Does it throw away any error below it?
        // An error below should push back the tokens,
        // So i think it is ok
        while let Ok(next_stmt) = self.parse_stmt() {
            if let Err(parse_error) = self.cmp_next_token(Token::SEMICOL) {
                return Err(parse_error);
            }
            stmts.push(next_stmt);
        }

        Ok(StmtList { stmts })
    }

    // Stmt ::= <DeclStmt> | <CtrlStmt>
    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let token = match self.lexer.next() {
            Some(t) => t,
            None => {
                return Err(ParseError {
                    msg: "Unexpected end of input.".to_string(),
                })
            }
        };

        let stmt = match token {
            Token::INT | Token::FLOAT | Token::BOOL | Token::STRING => {
                self.lexer.push_back(token);
                let decl_stmt = match self.parse_decl_stmt() {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(e);
                    }
                };
                Stmt::Decl(decl_stmt)
            }
            _ => {
                self.lexer.push_back(token);
                let ctrl_stmt = match self.parse_ctrl_stmt() {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(e)
                    }
                };
                Stmt::Ctrl(ctrl_stmt)
            }
        };

        Ok(stmt)
    }

    // DeclStmt ::= (INT | FLOAT | BOOL | STRING) IDENT { , IDENT }
    fn parse_decl_stmt(&mut self) -> Result<DeclStmt, ParseError> {
        let mut idents: Vec<Token> = vec![];

        let mut token = match self.lexer.next() {
            Some(t) => t,
            None => return Err(ParseError { msg: "Unexpected end of input".to_string() }),
        };

        let type_specifier = match token {
            Token::INT | Token::FLOAT | Token::BOOL | Token::STRING => token,
            _ => return Err(ParseError { msg: "Expected type specifier".to_string() }),
        };

        token = match self.lexer.next() {
            Some(t) => {
                match t {
                    Token::IDENT(name) => Token::IDENT(name),
                    _ => return Err(ParseError { msg: "Expected identifier.".to_string() })
                }
            }
            None => return Err(ParseError { msg: "Unexpected end of input".to_string() }),
        };

        idents.push(token);

        while let Ok(_) = self.cmp_next_token(Token::COMMA) {
            let ident = match self.cmp_next_token(Token::IDENT("".to_string())) {
                Ok(ident) => ident,
                Err(e) => return Err(e),
            };

            idents.push(ident);
        }

        Ok(DeclStmt {
            type_specifier,
            idents
        })
    }

    // CtrlStmt ::= <AssignStmt> | <PrintStmt> | <IfStmt> | <LoopStmt>
    fn parse_ctrl_stmt(&mut self) -> Result<CtrlStmt, ParseError> {
        Err(ParseError {
            msg: "Not implemented".to_string(),
        })
    }

    fn parse_assign_stmt(&mut self) -> Result<AssignStmt, ParseError> {
        Err(ParseError {
            msg: "Not implemented".to_string(),
        })
    }
    fn parse_print_stmt(&mut self) -> Result<AssignStmt, ParseError> {
        Err(ParseError {
            msg: "Not implemented".to_string(),
        })
    }
    fn parse_if_stmt(&mut self) -> Result<AssignStmt, ParseError> {
        Err(ParseError {
            msg: "Not implemented".to_string(),
        })
    }
    fn parse_loop_stmt(&mut self) -> Result<AssignStmt, ParseError> {
        Err(ParseError {
            msg: "Not implemented".to_string(),
        })
    }
}

pub struct Program {
    stmt_list: StmtList,
}

pub struct StmtList {
    stmts: Vec<Stmt>,
}

pub enum Stmt {
    Decl(DeclStmt),
    Ctrl(CtrlStmt),
}

pub struct DeclStmt {
    type_specifier: Token,
    idents: Vec<Token>,
}

pub enum CtrlStmt {
    Assign(AssignStmt),
    Print(PrintStmt),
    If(IfStmt),
    Loop(LoopStmt),
}

pub struct AssignStmt {
    ident: Token,
    expr: OrExpr,
}

pub struct PrintStmt {
    exprs: Vec<OrExpr>,
}

pub struct IfStmt {
    condition: OrExpr,
    then_stmts: StmtList,
    else_stmts: Option<StmtList>,
}

pub struct LoopStmt {
    condition: OrExpr,
    stmts: StmtList,
}

pub struct OrExpr {
    lhs: AndExpr,
    rhs: Vec<AndExpr>,
}

pub struct AndExpr {
    lhs: EqualExpr,
    rhs: Vec<EqualExpr>,
}

pub struct EqualExpr {
    lhs: RelExpr,
    rhs: Vec<RelExpr>,
}

pub struct RelExpr {
    lhs: AddExpr,
    rhs: Vec<AddExpr>,
}

pub struct AddExpr {
    lhs: MultExpr,
    rhs: Vec<MultExpr>,
}

pub struct MultExpr {
    lhs: UnaryExpr,
    rhs: Vec<UnaryExpr>,
}

pub struct UnaryExpr {
    op: Option<Token>,
    expr: PrimaryExpr,
}

pub struct PrimaryExpr {
    constant: Option<Token>,
    expr: Option<Box<OrExpr>>,
}
