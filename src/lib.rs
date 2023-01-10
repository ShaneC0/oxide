use std::fmt;

pub mod lexer;
pub mod parser;

#[derive(PartialEq)]
pub enum Token {
    INIT,
    HALT,
    PRINT,
    IF,
    THEN,
    ELSE,
    ENDIF,
    WHILE,
    DO,
    ENDWHILE,
    INT,
    FLOAT,
    BOOL,
    STRING,
    TRUE,
    FALSE,

    OPENPAREN,
    CLOSEPAREN,
    COMMA,
    SEMICOL,

    OROP,
    ANDOP,
    NOT,
    GTHAN,
    LTHAN,
    EQUALOP,
    ASSOP,
    PLUS,
    MINUS,
    MULT,
    DIV,
    MOD,

    ICONST(i32),
    FCONST(f64),
    BCONST(bool),
    SCONST(String),

    ERROR(String),
    DONE,

    IDENT(String),
}