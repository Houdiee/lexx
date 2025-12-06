#![allow(dead_code)]

use crate::{
    parser::{Expected, Parser, ParserError, ParserErrorKind},
    span::{Span, Spanned},
    token::{IdentKind, TokenKind},
};

pub type ASTNode<'src> = Spanned<AST<'src>>;

#[derive(Debug)]
pub enum AST<'src> {
    Literal {
        char: u8,
    },
    Reference {
        name: &'src [u8],
        kind: IdentKind,
    },
    Concat {
        left: Box<ASTNode<'src>>,
        right: Box<ASTNode<'src>>,
    },
    Alternation {
        left: Box<ASTNode<'src>>,
        right: Box<ASTNode<'src>>,
    },
    Repeat0 {
        inner: Box<ASTNode<'src>>,
    },
    Repeat1 {
        inner: Box<ASTNode<'src>>,
    },
    Optional {
        inner: Box<ASTNode<'src>>,
    },
    Group {
        inner: Box<ASTNode<'src>>,
    },
    Range {
        inner: Box<ASTNode<'src>>,
        start: usize,
        end: Option<usize>,
    },
    AnyChar,
}

impl<'src> Parser<'src> {
    // Regex ::= Alternation
    pub fn parse_regex(&mut self) -> Result<ASTNode<'src>, ParserError<'src>> {
        self.parse_alternation()
    }

    // Alternation ::= Concat { '|' Concat }
    fn parse_alternation(&mut self) -> Result<ASTNode<'src>, ParserError<'src>> {
        todo!()
    }

    // Concat ::= Quantifier { Quantifier }
    fn parse_concat(&mut self) -> Result<ASTNode<'src>, ParserError<'src>> {
        todo!()
    }

    // Quantifier ::= Atom { '*' | '+' | '?' | RangeRepetition }
    // RangeRepetition ::= '{' [ <int> ] ',' [ <int> ] '}'
    fn parse_quantifier(&mut self) -> Result<ASTNode<'src>, ParserError<'src>> {
        todo!()
    }

    // Atom ::=
    //     | <char>
    //     | '.'
    //     | '\' <char>
    //     | '{' <ident> '}'
    //     | '[' <char> ']
    //     | '(' regex ')'
    fn parse_atom(&mut self) -> Result<ASTNode<'src>, ParserError<'src>> {
        let peeked = self.peek_or_err()?;
        let span_start = peeked.span.start;

        match peeked.value {
            TokenKind::Literal { char } => {
                _ = self.consume();
                Ok(ASTNode {
                    value: AST::Literal { char },
                    span: peeked.span,
                })
            }

            TokenKind::Period => {
                _ = self.consume();
                Ok(ASTNode {
                    value: AST::AnyChar,
                    span: peeked.span,
                })
            }

            TokenKind::OpenParen => {
                _ = self.consume();
                let inner = self.parse_regex()?;
                let span_end = self.expect(Expected::ClosedParen)?.span.end;

                Ok(ASTNode {
                    value: AST::Group { inner: Box::new(inner) },
                    span: Span::from(span_start..span_end),
                })
            }

            TokenKind::OpenBrace => {
                _ = self.consume();
                let next_token = self.peek_or_err()?;
                match next_token.value {
                    TokenKind::Ident { name, kind } => {
                        todo!()
                    }

                    TokenKind::Number { .. } | TokenKind::Comma => {
                        todo!()
                    }

                    _ => {
                        Err(ParserError {
                            value: ParserErrorKind::UnexpectedEOF,
                            span: self.current_span(),
                        })
                    }
                }
            }

            _ => unreachable!("Invalid TokenKind was passed into parse_atom"),
        }
    }
}
