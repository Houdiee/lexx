use std::fs;

use crate::lexer::Lexer;

mod token;
mod span;
mod error;
mod warning;
mod lexer;
mod parser;
mod regex;

fn main() {
    let path = "./example/tokens.lexx";
    let source = fs::read_to_string(path).expect("Failed to read file");

    let lexer = Lexer::new(&source);
    for token in lexer {
        println!("{:?}", token);
    }
}
