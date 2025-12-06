#![allow(dead_code)]
use crate::{
    span::{Span, Spanned},
    token::{IdentKind, Token, TokenKind},
};

#[derive(Debug)]
pub struct Parser<'src> {
    tokens: &'src [Token<'src>],
    pos: usize,
}

#[derive(Debug)]
pub struct Rule<'src> {
    pub name: Spanned<&'src [u8]>,
    pub kind: IdentKind,
}

pub type ParserError<'src> = Spanned<ParserErrorKind<'src>>;

#[derive(Debug)]
pub enum ParserErrorKind<'src> {
    UnexpectedToken { expected: Expected, got: TokenKind<'src> },
    UnexpectedEOF,
}

#[derive(Debug)]
pub enum Expected {
    Ident,
    Equals,
    ClosedParen,
    Comma,
    ClosedBrace,
}

impl<'src> PartialEq<Expected> for TokenKind<'src> {
    fn eq(&self, other: &Expected) -> bool {
        match (self, other) {
            (TokenKind::Ident { .. }, Expected::Ident) => true,
            (TokenKind::Equals, Expected::Equals) => true,
            (TokenKind::ClosedParen, Expected::ClosedParen) => true,
            (TokenKind::Comma, Expected::Comma) => true,
            (TokenKind::ClosedBrace, Expected::ClosedBrace) => true,
            _ => false,
        }
    }
}

impl<'src> Iterator for Parser<'src> {
    type Item = Result<Rule<'src>, ParserError<'src>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peek().is_none() {
            return None;
        }
        Some(self.parse_rule())
    }
}

impl<'src> Parser<'src> {
    pub fn new(tokens: &'src [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn recover_from_error(&mut self) {
        _ = self.consume(); // make this shit better
    }

    pub fn parse(&mut self) -> (Vec<Rule<'src>>, Vec<ParserError<'src>>) {
        let mut rules = Vec::new();
        let mut errors = Vec::new();

        while let Some(rule_result) = self.next() {
            match rule_result {
                Ok(rule) => rules.push(rule),
                Err(err) => {
                    errors.push(err);
                    self.recover_from_error();
                }
            }
        }
        (rules, errors)
    }

    fn parse_rule(&mut self) -> Result<Rule<'src>, ParserError<'src>> {
        let ident_token = self.expect(Expected::Ident)?;
        let (ident_name, ident_kind) = match ident_token.value {
            TokenKind::Ident { name, kind } => (name, kind),
            _ => unreachable!(),
        };

        _ = self.expect(Expected::Equals)?;

        todo!()
    }

    pub fn current_span(&self) -> Span {
        if self.pos < self.tokens.len() {
            self.tokens[self.pos].span
        } else if self.tokens.len() > 0 {
            self.tokens[self.tokens.len() - 1].span
        } else {
            Span::default()
        }
    }

    pub fn expect(&mut self, expected: Expected) -> Result<Token<'src>, ParserError<'src>> {
        let consumed = self.consume().ok_or(ParserError {
            value: ParserErrorKind::UnexpectedEOF,
            span: self.current_span(),
        })?;

        if consumed.value != expected {
            return Err(ParserError {
                value: ParserErrorKind::UnexpectedToken {
                    expected,
                    got: consumed.value,
                },
                span: consumed.span,
            });
        }
        Ok(consumed)
    }

    pub fn peek_or_err(&mut self) -> Result<Token<'src>, ParserError<'src>> {
        self.peek().ok_or(ParserError {
            value: ParserErrorKind::UnexpectedEOF,
            span: self.current_span(),
        })
    }

    pub fn consume_or_err(&mut self) -> Result<Token<'src>, ParserError<'src>> {
        self.consume().ok_or(ParserError {
            value: ParserErrorKind::UnexpectedEOF,
            span: self.current_span(),
        })
    }

    pub fn peek(&self) -> Option<Token<'src>> {
        if self.pos < self.tokens.len() {
            Some(self.tokens[self.pos])
        } else {
            None
        }
    }

    pub fn consume(&mut self) -> Option<Token<'src>> {
        if self.pos < self.tokens.len() {
            let current = self.tokens[self.pos];
            self.pos += 1;
            Some(current)
        } else {
            None
        }
    }
}
