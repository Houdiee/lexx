use crate::span::Span;

#[derive(Debug)]
pub struct Rule<'src> {
    pub name: &'src str,
    pub name_span: Span,
    pub kind: RuleKind,
    pub pattern: ASTNode<'src>,
}

#[derive(Debug, Clone)]
pub enum RuleKind {
    Token,
    Helper,
    Skip,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ASTNode<'src> {
    Literal {
        char: char,
    },
    Reference {
        name: &'src str,
        span: Span,
    },
    Alternation {
        left: Box<ASTNode<'src>>,
        right: Box<ASTNode<'src>>,
    },
    Concat {
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
        span: Span,
    },
    Range {
        inner: Box<ASTNode<'src>>,
        min: usize,
        max: Option<usize>,
    },
    CharClass {
        negated: bool,
        parts: Vec<CharClassPart>,
    },
    AnyChar,
    Error,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CharClassPart {
    Literal { char: char },
    Range { min: char, max: char },
}
