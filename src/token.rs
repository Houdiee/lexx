use crate::span::Span;

#[derive(Debug, Clone, Copy)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind<'src> {
    Literal { char: char },
    Number { num: usize },
    Identifier { name: &'src str },
    ShorthandClass { char: char },
    Pipe,        // |
    Star,        // *
    Plus,        // +
    QMark,       // ?
    OpenBrace,   // {
    ClosedBrace, // }
    OpenBrack,   // [
    ClosedBrack, // ]
    OpenParen,   // (
    ClosedParen, // )
    Hyphen,      // -
    Comma,       // ,
    Period,      // .
    Caret,       // ^
    Dollar,      // $
    Equals,      // =
    Newline,     // \n
    Error,
}
