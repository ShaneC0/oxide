use std::{env, fs};

use oxide::{lexer::Lexer, Token, parser::Parser};

fn main() {
    let mut args = env::args();
    args.next();
    let file_path = args.next().expect("oxide: Missing filename.");
    let file = fs::read_to_string(file_path).expect("oxide: Couldn't open file.");
    let mut parser = Parser::new(&file);
    match parser.parse_program() {
        Ok(_) => (),
        Err(parse_error) => println!("{:#?}", parse_error)
    }
}
