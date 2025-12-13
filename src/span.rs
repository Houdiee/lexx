use miette::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<(usize, usize)> for Span {
    fn from(value: (usize, usize)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}

impl Span {
    pub fn to_source_span(self) -> SourceSpan {
        SourceSpan::from(self.start..self.end)
    }
}
