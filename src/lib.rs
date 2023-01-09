use std::iter::Peekable;
use std::str::Chars;
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

enum State {
    START,
    INID,
    ININT,
    INFLOAT,
    INSTRING,
    INCOMMENT,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: u32,
    pushed_back_token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input_str: &'a str) -> Self {
        Self {
            input: input_str.chars().peekable(),
            line: 1,
            pushed_back_token: None,
        }
    }

    pub fn push_back(&mut self, token: Token) {
        self.pushed_back_token = Some(token);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        //IF PUSHED BACK TOKEN IS SOME RETURN IT

        let mut state = State::START;
        let mut lexeme = String::from("");
        while let Some(ch) = self.input.next() {
            match state {
                State::START => {
                    if ch == ' ' {
                        continue;
                    }
                    if ch == '\n' {
                        self.line += 1;
                        continue;
                    }
                    if ch == '\"' {
                        state = State::INSTRING;
                        continue;
                    }
                    lexeme.push(ch);
                    if ch.is_numeric() {
                        state = State::ININT;
                        continue;
                    }
                    if ch.is_alphabetic() || ch == '_' {
                        state = State::INID;
                        continue;
                    }
                    let token = match ch {
                        '(' => Token::OPENPAREN,
                        ')' => Token::CLOSEPAREN,
                        ',' => Token::COMMA,
                        ';' => Token::SEMICOL,
                        '+' => Token::PLUS,
                        '-' => Token::MINUS,
                        '*' => Token::MULT,
                        '!' => Token::NOT,
                        '>' => Token::GTHAN,
                        '<' => Token::LTHAN,
                        '%' => Token::MOD,
                        // '&' => {

                        // }
                        // '|' => {

                        // }
                        // '/' => {

                        // }
                        // '=' => {

                        // }
                        _ => Token::ERROR(String::from("")),
                    };
                    return Some(token);
                }
                _ => (),
            }
        };
        None
    }
}
