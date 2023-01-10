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
    fn cmp_next_token(&mut self, target: Token) -> Option<ParseError> {
        let token_option = self.lexer.next();
        let token = match token_option {
            Some(t) => t,
            None => {
                return Some(ParseError {
                    msg: "Unexpected end of input.".to_string(),
                })
            }
        };

        if token == target {
            return None;
        }

        match token {
            Token::ERROR(msg) => Some(ParseError { msg }),
            _ => {
                self.lexer.push_back(token);
                Some(ParseError {
                    // How do i make it display the target token variant?
                    msg: "Expected token {placeholder)".to_string(),
                })
            }
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        if let Some(parse_error) = self.cmp_next_token(Token::INIT) {
            return Err(parse_error);
        }

        let stmt_list = match self.parse_stmt_list() {
            Ok(stmts) => stmts,
            Err(e) => return Err(e),
        };

        if let Some(parse_error) = self.cmp_next_token(Token::HALT) {
            return Err(parse_error);
        }

        Ok(Program { stmt_list })
    }

    fn parse_stmt_list(&mut self) -> Result<StmtList, ParseError> {
        let mut stmts: Vec<Stmt> = vec![];
        let stmt = match self.parse_stmt() {
            Ok(_stmt) => _stmt,
            Err(parse_error) => return Err(parse_error),
        };

        if let Some(parse_error) = self.cmp_next_token(Token::SEMICOL) {
            return Err(parse_error);
        }

        stmts.push(stmt);

        // This ignores when an error is returned.
        // Not sure if this is the right way to do this.
        // Does it throw away any error below it?
        // An error below should push back the tokens,
        // So i think it is ok
        while let Ok(next_stmt) = self.parse_stmt() {
            if let Some(parse_error) = self.cmp_next_token(Token::SEMICOL) {
                return Err(parse_error);
            }
            stmts.push(next_stmt);
        }

        Ok(StmtList { stmts })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
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
