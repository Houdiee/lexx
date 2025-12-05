use std::fs;

use crate::lexer::Lexer;

mod lexer;
mod span;
mod token;

fn main() {
    let file_path = "./example/tokens.lexx";
    let source = fs::read(file_path).expect("Failed to read file");
    let (tokens, lexer_errors) = Lexer::new(&source).lex();
    if !tokens.is_empty() {
        for token in tokens {
            println!("{token:?}");
        }
    }
    if !lexer_errors.is_empty() {
        for error in lexer_errors {
            println!("{error:?}");
        }
    }
}
