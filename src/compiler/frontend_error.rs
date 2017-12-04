use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FormatResult;

#[derive(Debug, Clone, Copy)]
pub enum FrontendError {
    //general
    EOI,
    Default,

    //lexical
    ExpectedArrowTip,
    ExpectedMoreInput,
    ExpectedEscapeSequence,
    ExpectedIdentifier,
    UnrecognizedInput,
    ExpectedColon,

    //syntactical
    ExpectedProdStartOrUse,
    ExpectedName,
    ExpectedArrow,
    ExpectedEndl,
    ExpectedUse,
    ExpectedCloseCurlyBrace,
    ExpectedPart,
}

impl Error for FrontendError {
    fn description(&self) -> &str {
        match *self {
            FrontendError::ExpectedArrowTip => "expected '>'",
            FrontendError::ExpectedCloseCurlyBrace => "expected '}'",
            FrontendError::ExpectedMoreInput => "expected more input",
            FrontendError::ExpectedEscapeSequence => "expected escape sequence",
            FrontendError::ExpectedIdentifier => "expected identifer",
            FrontendError::UnrecognizedInput => "unrecognized input",
            FrontendError::ExpectedColon => "expected ':'",
            FrontendError::EOI => "end of input",
            FrontendError::Default => "default",
            FrontendError::ExpectedProdStartOrUse => "expected production name or 'use'",
            FrontendError::ExpectedName => "expected Name",
            FrontendError::ExpectedArrow => "expected ->",
            FrontendError::ExpectedEndl => "expected endline",
            FrontendError::ExpectedUse => "expected 'use'",
            FrontendError::ExpectedPart => {
                "expected one of: LITERAL, LEXICAL RULE NAME, NAME, EPSILON"
            }
        }
    }
}

impl Display for FrontendError {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "Lexical Error: {}", self.description())
    }
}