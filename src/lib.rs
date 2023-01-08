use std::iter::Peekable;
use std::str::Chars;
enum Token {
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

    ICONST,
    FCONST,
    BCONST,
    SCONST,

    ERROR,
    DONE,

    IDENT,
}

enum State {
    START,
    INID,
    ININT,
    INFLOAT,
    INSTRING,
    INCOMMENT,
}

pub struct Lexeme {
    token: Token,
    lexeme: String,
    line: u32,
}

impl Lexeme {
    fn new(token: Token, lexeme: String, line: u32) -> Self {
        Self {
            token,
            lexeme,
            line,
        }
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: u32,
    pushed_back_lexeme: Option<Lexeme>,
}

impl<'a> Lexer<'a> {
    pub fn new(input_str: &'a str) -> Self {
        Self {
            input: input_str.chars().peekable(),
            line: 1,
            pushed_back_lexeme: None,
        }
    }

    pub fn push_back(&mut self, lexeme: Lexeme) {
        self.pushed_back_lexeme = Some(lexeme);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        //if not none return the field itself?
        match self.pushed_back_lexeme {
            Some(lexeme) => {
                self.pushed_back_lexeme = None;
                return Some(lexeme);
            }
            None => (),
        };

        let mut state = State::START;
        let mut lexeme = String::from("");
        while let Some(ch) = self.input.next() {
            if ch == ' ' {
                continue;
            }
            if ch == '\n' {
                self.line += 1;
                continue;
            }
            match state {
                State::START => {
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
                    let tk = match ch {
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
                        '&' => {

                        }
                        '|' => {

                        }
                        '/' => {

                        }
                        '=' => {

                        }
                        _ => Token::ERROR,
                    };
                    return Some(Lexeme::new(tk, lexeme, self.line));
                }
                _ => (),
            }
        };
    }
}
