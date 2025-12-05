use std::fs;

use crate::lexer::Lexer;

mod lexer;
mod span;
mod token;

fn main() {
    let file_path = "./example/tokens.lexx";
    let source = fs::read(file_path).expect("Failed to read file");
    let (tokens, lexer_errors) = Lexer::new(&source).lex();
}
