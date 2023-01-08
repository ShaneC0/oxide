use std::{env, fs};
use oxide::Lexer;

fn main() {
    let mut args = env::args();
    args.next();
    let file_path = match args.next() {
        Some(path) => path,
        None => panic!("oxide: Missing filename."),
    };

    let file = fs::read_to_string(file_path).expect("oxide: Couldn't open file.");

    let lexer = Lexer::new(&file);

}
