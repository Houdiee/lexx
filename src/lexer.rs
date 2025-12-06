#![allow(dead_code)]
use crate::{
    span::{Span, Spanned},
    token::{IdentKind, Token, TokenKind},
};

#[derive(Debug)]
pub struct Lexer<'src> {
    source: &'src [u8],
    pos: usize,
    is_in_braces: bool,
    is_in_brackets: bool,
    is_expecting_expr: bool,
}

pub type LexerError<'src> = Spanned<LexerErrorKind<'src>>;

#[derive(Debug)]
pub enum LexerErrorKind<'src> {
    NumberOverflow,
    InvalidEscape { escaped: &'src [u8] },
    InvalidCharacter { char: u8 },
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token<'src>, LexerError<'src>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let span_start = self.pos;
        let peeked = self.peek()?;

        if !peeked.is_ascii() {
            _ = self.consume();
            return Some(Err(LexerError {
                value: LexerErrorKind::InvalidCharacter { char: peeked },
                span: Span::from(span_start..self.pos),
            }));
        }

        let token_kind = match peeked {
            b'(' => {
                _ = self.consume();
                TokenKind::OpenParen
            }
            b')' => {
                _ = self.consume();
                TokenKind::ClosedParen
            }
            b'[' => {
                _ = self.consume();
                self.is_in_brackets = true;
                TokenKind::OpenParen
            }
            b']' => {
                _ = self.consume();
                self.is_in_brackets = false;
                TokenKind::ClosedBrack
            }
            b'{' => {
                _ = self.consume();
                self.is_in_braces = true;
                TokenKind::OpenBrace
            }
            b'}' => {
                _ = self.consume();
                self.is_in_braces = false;
                TokenKind::ClosedBrace
            }
            b'=' if !self.is_expecting_expr => {
                _ = self.consume();
                self.is_expecting_expr = true;
                TokenKind::Equals
            }
            b'-' => {
                _ = self.consume();
                TokenKind::Hyphen
            }
            b'|' => {
                _ = self.consume();
                TokenKind::Pipe
            }
            b'*' => {
                _ = self.consume();
                TokenKind::Star
            }
            b'+' => {
                _ = self.consume();
                TokenKind::Plus
            }
            b'?' => {
                _ = self.consume();
                TokenKind::QMark
            }
            b',' => {
                _ = self.consume();
                TokenKind::Comma
            }
            b'.' => {
                _ = self.consume();
                TokenKind::Period
            }
            b'^' => {
                _ = self.consume();
                TokenKind::Caret
            }
            b'$' => {
                _ = self.consume();
                TokenKind::Dollar
            }
            byte if self.is_in_braces && byte.is_ascii_digit() => {
                let number = match self.consume_number() {
                    Ok(val) => val,
                    Err(e) => return Some(Err(e)),
                };
                TokenKind::Number { val: number }
            }
            byte if self.is_in_braces && byte.is_ascii_alphabetic() => {
                let ident_name = self.consume_ident();
                let ident_kind = Self::get_ident_kind(ident_name);
                TokenKind::Ident {
                    name: ident_name,
                    kind: ident_kind,
                }
            }
            byte if !self.is_expecting_expr && (byte.is_ascii_alphabetic() || byte == b'_') => {
                let ident_name = self.consume_ident();
                let ident_kind = Self::get_ident_kind(ident_name);
                TokenKind::Ident {
                    name: ident_name,
                    kind: ident_kind,
                }
            }
            b' ' if self.is_in_brackets => {
                _ = self.consume();
                TokenKind::Literal { char: peeked }
            }
            b'\n' => {
                while self.peek().is_some_and(|c| c == b'\n') {
                    _ = self.consume();
                }
                self.is_expecting_expr = false;
                TokenKind::Newline
            }
            b'\\' => {
                _ = self.consume();
                todo!()
            }
            _ => {
                _ = self.consume();
                TokenKind::Literal { char: peeked }
            }
        };
        let span_end = self.pos;

        Some(Ok(Token {
            value: token_kind,
            span: Span::from(span_start..span_end),
        }))
    }
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src [u8]) -> Self {
        Self {
            source,
            pos: 0,
            is_in_braces: false,
            is_in_brackets: false,
            is_expecting_expr: false,
        }
    }

    pub fn lex(&mut self) -> (Vec<Token<'src>>, Vec<LexerError<'src>>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while let Some(token_result) = self.next() {
            match token_result {
                Ok(token) => tokens.push(token),
                Err(err) => errors.push(err),
            }
        }
        (tokens, errors)
    }

    fn consume_ident(&mut self) -> &'src [u8] {
        let span_start = self.pos;
        if self.peek().is_some_and(|c| c.is_ascii_alphabetic() || c == b'_') {
            _ = self.consume();
        }
        while self.peek().is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_') {
            _ = self.consume();
        }
        let span_end = self.pos;
        &self.source[span_start..span_end]
    }

    fn get_ident_kind(ident_name: &'src [u8]) -> IdentKind {
        let first_char = *ident_name.first().expect("An empty string was passed into get_ident_kind");
        match first_char {
            ch if ch.is_ascii_uppercase() => IdentKind::Token,
            ch if ch.is_ascii_lowercase() => IdentKind::Helper,
            ch if ch == b'_' => IdentKind::Ignore,
            _ => panic!("Invalid ident_name was passed into get_ident_kind"),
        }
    }

    fn consume_number(&mut self) -> Result<usize, LexerError<'src>> {
        let span_start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            _ = self.consume();
        }
        let span_end = self.pos;
        let number_slice = &self.source[span_start..span_end];

        let mut value: usize = 0;
        for &byte in number_slice {
            let digit = (byte - b'0') as usize;

            value = value.checked_mul(10).ok_or(LexerError {
                value: LexerErrorKind::NumberOverflow,
                span: Span::from(span_start..span_end),
            })?;

            value = value.checked_add(digit).ok_or(LexerError {
                value: LexerErrorKind::NumberOverflow,
                span: Span::from(span_start..span_end),
            })?;
        }
        Ok(value)
    }

    fn skip_whitespace(&mut self) {
        if self.is_in_brackets {
            return;
        }

        while self.peek().is_some_and(|c| c == b' ' || c == b'\t' || c == b'\r') {
            _ = self.consume();
        }
    }

    fn peek(&self) -> Option<u8> {
        if self.pos < self.source.len() {
            Some(self.source[self.pos])
        } else {
            None
        }
    }

    fn consume(&mut self) -> Option<u8> {
        if self.pos < self.source.len() {
            let current = self.source[self.pos];
            self.pos += 1;
            Some(current)
        } else {
            None
        }
    }
}
