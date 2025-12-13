use std::fs;

use miette::{Report, NamedSource};

use crate::lexer::Lexer;

mod error;
mod lexer;
mod ast;
mod parser;
mod regex;
mod span;
mod token;
mod warning;

fn main() {
    let path = "./example/tokens.lexx";
    let source = fs::read_to_string(path).expect("Failed to read file");
    let named_source = NamedSource::new(path, source.clone());

    let mut lexer = Lexer::new(&source);
    let (tokens, errors) = lexer.lex();
    for error in errors {
        let report = Report::new(error).with_source_code(named_source.clone());
        eprintln!("{:?}", report);
    }
}
