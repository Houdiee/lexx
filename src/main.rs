use std::fs;

use crate::{lexer::Lexer, parser::Parser};

mod lexer;
mod parser;
mod span;
mod token;
mod regex;

fn main() {
    let file_path = "./example/tokens.lexx";
    let source = fs::read(file_path).expect("Failed to read file");

    let (tokens, lexer_errors) = Lexer::new(&source).lex();
    if !tokens.is_empty() {
        // for token in &tokens {
        //     println!("{token:?}");
        // }
    }
    if !lexer_errors.is_empty() {
        // for error in &lexer_errors {
        //     println!("{error:?}");
        // }
    }

    let (rules, parser_errors) = Parser::new(&tokens).parse();
    if !rules.is_empty() {
        for rule in &rules {
            println!("{rule:?}");
        }
    }
    if !parser_errors.is_empty() {
        for error in &parser_errors {
            println!("{error:?}");
        }
    }
}
