use miette::{Diagnostic, LabeledSpan, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Warning")]
pub struct Warning {
    #[source]
    pub kind: WarningKind,
    pub span: SourceSpan,
}

#[derive(Error, Diagnostic, Debug)]
pub enum WarningKind {
    #[error("Identical Rule Patterns")]
    #[diagnostic(help("Consider removing the paranthesis"), severity(warning))]
    IdenticalRulePatterns {
        name1: String,
        span1: SourceSpan,
        name2: String,
        span2: SourceSpan,
    },

    #[error("Unnecessary Paranthesis")]
    #[diagnostic(help("Consider removing the paranthesis"), severity(warning))]
    UnnecessaryParanthesis,

    #[error("Unused Helper Rule")]
    #[diagnostic(help("Consider removing this rule"), severity(warning))]
    UnusedHelperRule,

    #[error("Unnecessary Range Boundary")]
    #[diagnostic(help("Conside replacing {{{value},{value}}} with {{{value}}}"), severity(warning))]
    UnnecessaryRangeBoundary { value: usize },
}

impl Diagnostic for Warning {
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
            WarningKind::IdenticalRulePatterns {
                name1: _,
                span1,
                name2: _,
                span2,
            } => {
                let text1 = String::from("first definition here");
                let label1 = LabeledSpan::new_with_span(Some(text1), span1);

                let text2 = String::from("second definition here");
                let label2 = LabeledSpan::new_with_span(Some(text2), span2);
                Some(Box::new(std::iter::once(label1).chain(std::iter::once(label2))))
            }

            _ => {
                let text = String::from("here");
                let label = LabeledSpan::new_with_span(Some(text), self.span);
                Some(Box::new(std::iter::once(label)))
            }
        }
    }
}
