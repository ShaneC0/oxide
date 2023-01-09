use crate::{lexer::Lexer, Token};

struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    fn parse(&mut self) -> Program {
    }
}

struct Program {
    children: Vec<String>
}