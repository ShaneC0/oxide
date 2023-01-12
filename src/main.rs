use std::{env, fs};

use oxide::{Token, lexer::Lexer};

fn main() {
    let mut args = env::args();
    args.next();
    let file_path = args.next().expect("oxide: Missing filename.");
    let file = fs::read_to_string(file_path).expect("oxide: Couldn't open file.");
    // let mut parser = Parser::new(&file);
    // let program = match parser.parse_program() {
    //     Ok(prog) => prog,
    //     Err(parse_error) => panic!("{:#?}", parse_error),
    // };
    let mut lexer = Lexer::new(&file);
    let tok = Token::IDENT("Hello".to_string());
    lexer.push_back(tok);
    let tok2 = lexer.next().unwrap();
    println!("{:#?}", tok2);
    match lexer.next() {
        Some(t) => println!("{:?}", t),
        None => ()
    }
    
}
