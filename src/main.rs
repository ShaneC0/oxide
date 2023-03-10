use std::{env, fs};

use oxide::{lexer::Lexer, parser::Parser, Token};

fn main() {
    let mut args = env::args();
    args.next();
    let file_path = args.next().expect("oxide: Missing filename.");
    let file = fs::read_to_string(file_path).expect("oxide: Couldn't open file.");
    let mut parser = Parser::new(&file);
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(parse_error) => panic!("{:#?}", parse_error),
    };
    println!("Parsed program: {:#?}", program);
}
