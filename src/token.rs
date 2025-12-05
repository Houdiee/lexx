use crate::{span::Spanned};

pub type Token<'src> = Spanned<TokenKind<'src>>;

#[derive(Debug, Clone, Copy)]
pub enum TokenKind<'src> {
    Literal { char: u8 },
    Escaped { char: &'src [u8] },
    Number { val: usize },
    Reference { name: &'src [u8] },
    Ident { name: &'src [u8], kind: IdentKind },
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

#[derive(Debug, Clone, Copy)]
pub enum IdentKind {
    Token,
    Helper,
    Ignore,
}
