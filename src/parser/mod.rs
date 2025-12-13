use crate::{ast::ASTNode, error::Error, token::{Token, TokenKind}, warning::Warning};

mod regex;

#[derive(Debug)]
pub struct Parser<'src> {
    tokens: &'src [Token<'src>],
    index: usize,
    errors: Vec<Error>,
    warnings: Vec<Warning>,
}

impl<'src> Iterator for Parser<'src> {
    type Item = ASTNode<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'src> Parser<'src> {
    pub fn new(tokens: &'src [Token]) -> Self {
        Self {
            tokens,
            index: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn peek(&self) -> Option<Token<'src>> {
        if self.index < self.tokens.len() {
            Some(self.tokens[self.index])
        } else {
            None
        }
    }

    fn consume(&mut self) -> Option<Token<'src>> {
        if self.index < self.tokens.len() {
            let current = self.tokens[self.index];
            self.index += 1;
            Some(current)
        } else {
            None
        }
    }
}
