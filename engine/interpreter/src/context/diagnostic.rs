use std::fmt;

use diagnostics::*;
use parser::*;

use crate::SemanticError;

#[derive(Debug)]
pub enum DiagnosticError {
    Parser(ParserError),
    Semantic(SemanticError),
}

impl DiagnosticError {
    pub fn from_parser_err(err: LinkedErr<ParserError>) -> LinkedErr<DiagnosticError> {
        let LinkedErr { link, e } = err;
        LinkedErr {
            link,
            e: DiagnosticError::Parser(e),
        }
    }
    pub fn from_semantic_err(err: LinkedErr<SemanticError>) -> LinkedErr<DiagnosticError> {
        let LinkedErr { link, e } = err;
        LinkedErr {
            link,
            e: DiagnosticError::Semantic(e),
        }
    }
}

impl ErrorCode for DiagnosticError {
    fn code(&self) -> &'static str {
        match self {
            Self::Parser(err) => err.code(),
            Self::Semantic(err) => err.code(),
        }
    }

    fn src(&self) -> ErrorSource {
        match self {
            Self::Parser(err) => err.src(),
            Self::Semantic(err) => err.src(),
        }
    }
}

impl fmt::Display for DiagnosticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parser(err) => write!(f, "{err}"),
            Self::Semantic(err) => write!(f, "{err}"),
        }
    }
}
