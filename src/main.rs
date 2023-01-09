use std::{env, fs};
use oxide::Lexer;
use oxide::Token;

fn main() {
    let mut args = env::args();
    args.next();
    let file_path = args.next().expect("oxide: Missing filename.");
    let file = fs::read_to_string(file_path).expect("oxide: Couldn't open file.");

    let mut lexer = Lexer::new(&file);

    while let Some(token) = lexer.next() {
        println!("{:?}", token);

        if let Token::ERROR(_) = token {
            break;
        }
    }
}