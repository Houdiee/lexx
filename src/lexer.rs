use std::{
    iter::Peekable,
    ops::{Add, Mul},
    str::Chars,
};

use crate::{
    error::{Error, ErrorKind},
    regex::{control_to_literal, is_control_char, is_escaped_literal, is_shorthand_class},
    span::Span,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Lexer<'src> {
    source: Peekable<Chars<'src>>,
    bytes: &'src [u8],
    offset: usize,
    is_in_braces: bool,
    is_in_brackets: bool,
    is_expecting_expr: bool,
    errors: Vec<Error>,
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.skip_whitespace();
            let span_start = self.offset;
            let peeked = *self.peek()?;
            let token_kind = match peeked {
                '|' => {
                    self.consume();
                    TokenKind::Pipe
                }
                '*' => {
                    self.consume();
                    TokenKind::Star
                }
                '+' => {
                    self.consume();
                    TokenKind::Plus
                }
                '?' => {
                    self.consume();
                    TokenKind::QMark
                }
                '{' => {
                    self.consume();
                    self.is_in_braces = true;
                    TokenKind::OpenBrace
                }
                '}' => {
                    self.consume();
                    self.is_in_braces = false;
                    TokenKind::ClosedBrace
                }
                '[' => {
                    self.consume();
                    self.is_in_brackets = true;
                    TokenKind::OpenBrack
                }
                ']' => {
                    self.consume();
                    self.is_in_brackets = false;
                    TokenKind::ClosedBrack
                }
                '(' => {
                    self.consume();
                    TokenKind::OpenParen
                }
                ')' => {
                    self.consume();
                    TokenKind::ClosedParen
                }
                '$' => {
                    self.consume();
                    TokenKind::Dollar
                }
                '^' if self.is_in_brackets => {
                    self.consume();
                    TokenKind::Caret
                }
                '-' if self.is_in_brackets => {
                    self.consume();
                    TokenKind::Hyphen
                }
                ',' if self.is_in_braces => {
                    self.consume();
                    TokenKind::Comma
                }
                '=' if !self.is_expecting_expr => {
                    self.consume();
                    self.is_expecting_expr = true;
                    TokenKind::Equals
                }

                ch if !self.is_expecting_expr && (ch.is_alphabetic() || ch == '_') => {
                    let name = self.consume_identifier();
                    TokenKind::Identifier { name }
                }

                ch if self.is_in_braces && ch.is_alphabetic() => {
                    let name = self.consume_identifier();
                    TokenKind::Identifier { name }
                }

                ch if self.is_in_braces && ch.is_ascii_digit() => {
                    let Some(num) = self.consume_number() else {
                        let span_end = self.offset;
                        return Some(Token {
                            kind: TokenKind::Error,
                            span: Span::from((span_start, span_end)),
                        });
                    };
                    TokenKind::Number { num }
                }

                '\t' if self.is_in_brackets => {
                    self.consume();
                    let span = Span::from((span_start, self.offset));
                    self.errors.push(Error {
                        kind: ErrorKind::LiteralTabCharacter,
                        span,
                    });
                    return Some(Token {
                        kind: TokenKind::Error,
                        span,
                    });
                }

                '\n' => {
                    self.consume();
                    self.is_expecting_expr = false;
                    TokenKind::Newline
                }

                '\\' => return Some(self.tokenize_escape()),

                '.' => {
                    self.consume();
                    TokenKind::Period
                }

                _ => {
                    self.consume();
                    TokenKind::Literal { char: peeked }
                }
            };
            let span_end = self.offset;

            Some(Token {
                kind: token_kind,
                span: Span::from((span_start, span_end)),
            })
        }
    }
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source: source.chars().peekable(),
            bytes: source.as_bytes(),
            offset: 0,
            is_in_braces: false,
            is_in_brackets: false,
            is_expecting_expr: false,
            errors: Vec::new(),
        }
    }

    unsafe fn tokenize_escape(&mut self) -> Token<'src> {
        unsafe {
            let span_start = self.offset;
            self.consume();

            let Some(&escaped) = self.peek() else {
                let span = Span::from((span_start, self.offset));
                self.errors.push(Error {
                    kind: ErrorKind::UnexpectedEOF,
                    span,
                });
                return Token {
                    kind: TokenKind::Error,
                    span,
                };
            };

            let token_kind = match escaped {
                'x' => return self.tokenize_unicode_escape('x', 2),

                'u' => return self.tokenize_unicode_escape('u', 4),


                'U' => return self.tokenize_unicode_escape('U', 8),

                char if is_escaped_literal(char) => {
                    self.consume();
                    TokenKind::Literal { char }
                }

                char if is_control_char(char) => {
                    self.consume();
                    let literal = control_to_literal(char).unwrap_unchecked();
                    TokenKind::Literal { char: literal }
                }

                char if is_shorthand_class(char) => {
                    self.consume();
                    TokenKind::ShorthandClass { char }
                }

                _ => {
                    self.consume();
                    let span = Span::from((span_start, self.offset));
                    self.errors.push(Error {
                        kind: ErrorKind::InvalidEscapeCharacter { char: escaped },
                        span,
                    });
                    TokenKind::Error
                }
            };

            let span_end = self.offset;
            Token {
                kind: token_kind,
                span: Span::from((span_start, span_end)),
            }
        }
    }

    unsafe fn tokenize_unicode_escape(&mut self, escape_char: char, required_digits: usize) -> Token<'src> {
        unsafe {
            let span_start = self.offset - '\\'.len_utf8();
            self.consume();
            let (value, consumed) = self.consume_hex(required_digits);
            let span = Span::from((span_start, self.offset));

            if consumed != required_digits {
                self.errors.push(Error {
                    kind: ErrorKind::InvalidHexEscape {
                        char: escape_char,
                        expected_digits: required_digits,
                    },
                    span,
                });
                return Token {
                    kind: TokenKind::Error,
                    span,
                };
            }

            let Some(char) = char::from_u32(value) else {
                self.errors.push(Error {
                    kind: ErrorKind::InvalidUnicodeScalar { value },
                    span,
                });
                return Token {
                    kind: TokenKind::Error,
                    span,
                };
            };

            Token {
                kind: TokenKind::Literal { char },
                span,
            }
        }
    }

    unsafe fn consume_hex(&mut self, required_digits: usize) -> (u32, usize) {
        let mut value: u32 = 0;
        let mut consumed_count = 0;

        for _ in 0..required_digits {
            let Some(digit_char) = self.peek().copied() else {
                break;
            };
            let Some(digit) = digit_char.to_digit(16) else {
                break;
            };

            value = value.mul(16).add(digit);
            self.consume();
            consumed_count += 1;
        }
        (value, consumed_count)
    }

    unsafe fn consume_identifier(&mut self) -> &'src str {
        unsafe {
            let span_start = self.offset;
            if self.peek().is_some_and(|&c| c.is_alphabetic() || c == '_') {
                self.consume();
            }
            while self.peek().is_some_and(|&c| c.is_alphanumeric() || c == '_') {
                self.consume();
            }
            let span_end = self.offset;
            let slice = &self.bytes[span_start..span_end];
            str::from_utf8_unchecked(slice)
        }
    }

    fn consume_number(&mut self) -> Option<usize> {
        let span_start = self.offset;
        while self.peek().is_some_and(|&c| c.is_ascii_digit()) {
            self.consume();
        }

        let span_end = self.offset;
        if span_start == span_end {
            return None;
        }

        let number_slice = &self.bytes[span_start..span_end];
        let mut value: usize = 0;
        for byte in number_slice {
            let digit = (byte - b'0') as usize;

            let Some(val) = value.checked_mul(10) else {
                self.errors.push(Error {
                    kind: ErrorKind::IntegerOverflow,
                    span: Span::from((span_start, span_end)),
                });
                return None;
            };
            value = val;

            let Some(val) = value.checked_add(digit) else {
                self.errors.push(Error {
                    kind: ErrorKind::IntegerOverflow,
                    span: Span::from((span_start, span_end)),
                });
                return None;
            };
            value = val;
        }
        Some(value)
    }

    fn skip_whitespace(&mut self) {
        if self.is_in_brackets {
            return;
        }
        while self.peek().is_some_and(|c| c.is_whitespace()) {
            self.consume();
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn consume(&mut self) -> Option<char> {
        match self.source.next() {
            Some(char) => {
                self.offset += char.len_utf8();
                Some(char)
            }
            None => None,
        }
    }
}
