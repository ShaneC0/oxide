use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
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

    fn error(&self, lexeme: String) -> Token {
        Token::ERROR(String::from(format!(
            "Unrecognized token at line {}: \'{}\'.",
            self.line, lexeme
        )))
    }

    // Checks if next char is the same, if so advances iterator
    fn cmp_next_char(&mut self, current_char: &char) -> bool {
        if let Some(&next_char) = self.input.peek() {
            if *current_char == next_char {
                self.input.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        //IF PUSHED BACK TOKEN IS SOME RETURN IT

        let mut state = State::START;
        let mut lexeme = String::from("");
        let mut digit_after_decimal_seen = false;
        let mut digit_before_decimal_seen = false;
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
                        '&' => {
                            if self.cmp_next_char(&ch) {
                                return Some(Token::ANDOP);
                            } else {
                                return Some(self.error(lexeme));
                            }
                        }
                        '|' => {
                            if self.cmp_next_char(&ch) {
                                return Some(Token::OROP);
                            } else {
                                return Some(self.error(lexeme));
                            }
                        }
                        '/' => {
                            if self.cmp_next_char(&ch) {
                                state = State::INCOMMENT;
                                continue;
                            } else {
                                return Some(Token::DIV);
                            }
                        }
                        '=' => {
                            if self.cmp_next_char(&ch) {
                                return Some(Token::EQUALOP);
                            } else {
                                return Some(Token::ASSOP);
                            }
                        }
                        _ => self.error(lexeme),
                    };
                    return Some(token);
                }
                State::INCOMMENT => {
                    if ch == '\n' {
                        state = State::START;
                    }
                }
                State::ININT => {
                    if ch.is_numeric() {
                        lexeme.push(ch);
                        digit_before_decimal_seen = true;
                    } else if ch == '.' {
                        if digit_before_decimal_seen {
                            lexeme.push(ch);
                            state = State::INFLOAT;
                        } else {
                            return Some(self.error(lexeme));
                        }
                    } else {
                        //need to pushback the char here.
                        return Some(Token::ICONST(lexeme.parse::<i32>().unwrap()));
                    }
                }
                State::INFLOAT => {
                    if ch.is_numeric() {
                        lexeme.push(ch);
                        digit_after_decimal_seen = true;
                    } else if digit_after_decimal_seen {
                        //Need to push back char here.
                        return Some(Token::FCONST(lexeme.parse::<f64>().unwrap()));
                    } else {
                        return Some(self.error(lexeme));
                    }
                }
                State::INSTRING => {
                    if ch == '\"' {
                        return Some(Token::SCONST(lexeme));
                    } else if ch == '\n' {
                        return Some(self.error(lexeme));
                    } else {
                        lexeme.push(ch);
                    }
                }
                State::INID => {
                    if ch.is_alphanumeric() || ch == '_' {
                        lexeme.push(ch);
                    } else {
                        //need to push back ch here
                        return Some(match lexeme.as_str() {
                            "init" => Token::INIT,
                            "halt" => Token::HALT,
                            "print" => Token::PRINT,
                            "if" => Token::IF,
                            "then" => Token::THEN,
                            "else" => Token::ELSE,
                            "endif" => Token::ENDIF,
                            "while" => Token::WHILE,
                            "do" => Token::DO,
                            "endwhile" => Token::ENDWHILE,
                            "int" => Token::INT,
                            "float" => Token::FLOAT,
                            "bool" => Token::BOOL,
                            "string" => Token::STRING,
                            "true" => Token::BCONST(true),
                            "false" => Token::BCONST(false),
                            _ => Token::IDENT(lexeme),
                        });
                    }
                }
            }
        }

        // Return the last token in the case of end of input.
        match state {
            State::START => None,
            State::INCOMMENT => None,
            State::INID => Some(match lexeme.as_str() {
                "init" => Token::INIT,
                "halt" => Token::HALT,
                "print" => Token::PRINT,
                "if" => Token::IF,
                "then" => Token::THEN,
                "else" => Token::ELSE,
                "endif" => Token::ENDIF,
                "while" => Token::WHILE,
                "do" => Token::DO,
                "endwhile" => Token::ENDWHILE,
                "int" => Token::INT,
                "float" => Token::FLOAT,
                "bool" => Token::BOOL,
                "string" => Token::STRING,
                "true" => Token::BCONST(true),
                "false" => Token::BCONST(false),
                _ => Token::IDENT(lexeme),
            }),
            State::ININT => Some(Token::ICONST(lexeme.parse::<i32>().unwrap())),
            State::INFLOAT => {
                if digit_after_decimal_seen {
                    Some(Token::FCONST(lexeme.parse::<f64>().unwrap()))
                } else {
                    Some(self.error(lexeme))
                }
            }
            State::INSTRING => Some(Token::ERROR(String::from(format!(
                "Missing closing quotation at line {}.",
                self.line
            )))),
        }
    }
}
