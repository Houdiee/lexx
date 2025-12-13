use miette::{Diagnostic, LabeledSpan};
use thiserror::Error;

use crate::span::Span;

#[derive(Debug, Error)]
#[error("Error")]
pub struct Error {
    #[source]
    pub kind: ErrorKind,
    pub span: Span,
}

#[derive(Debug, Error, Diagnostic)]
pub enum ErrorKind {
    #[error("Invalid Unicode Scalar")]
    #[diagnostic(help("The scalar value '{value}' does not correspond to a valid UTF-8 character"))]
    InvalidUnicodeScalar { value: u32 },

    #[error("Invalid Hex Escape")]
    #[diagnostic(help("The escape '\\{char}' must be followed by {expected_digits} digits"))]
    InvalidHexEscape { char: char, expected_digits: usize },

    #[error("Invalid Escape Character")]
    #[diagnostic(help("The escape '\\{char}' is not a valid escape character"))]
    InvalidEscapeCharacter { char: char},

    #[error("Literal Tab Character")]
    #[diagnostic(help("Replace the literal tab character with '\\t'"))]
    LiteralTabCharacter,

    #[error("Integer Overflow")]
    #[diagnostic(help("The value is too large, please pick a smaller value"))]
    IntegerOverflow,

    #[error("Unexpected End-of-File")]
    UnexpectedEOF,
}

impl Diagnostic for Error {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.kind.code()
    }
    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.kind.help()
    }
    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.kind.url()
    }
    fn severity(&self) -> Option<miette::Severity> {
        self.kind.severity()
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match self.kind {
            _ => {
                let text = String::from("here");
                let label = LabeledSpan::new_with_span(Some(text), self.span.to_source_span());
                Some(Box::new(std::iter::once(label)))
            }
        }
    }
}
