use crate::span::Spanned;

pub type Token<'src> = Spanned<TokenKind<'src>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind<'src> {
    Literal { char: u8 },
    Number { val: usize },
    Ident { name: &'src [u8], kind: IdentKind },
    Escaped { sequence: &'src [u8] },
    ShorthandClass { char: u8 },
    OpenParen,   // (
    ClosedParen, // )
    OpenBrack,   // [
    ClosedBrack, // ]
    OpenBrace,   // {
    ClosedBrace, // }
    Hyphen,      // -
    Pipe,        // |
    Star,        // *
    Plus,        // +
    QMark,       // ?
    Comma,       // ,
    Period,      // .
    Caret,       // ^
    Dollar,      // $
    Equals,      // =
    Newline,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IdentKind {
    Token,
    Helper,
    Ignore,
}
