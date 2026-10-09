use std::fmt;

use diagnostics::*;
use lint::LintError;
use parser::*;

use crate::SemanticError;

#[derive(Debug)]
pub enum DiagnosticError {
    Parser(ParserError),
    Semantic(SemanticError),
    Lint(LintError),
}

impl DiagnosticError {
    pub fn from_parser_err(err: LinkedErr<ParserError>) -> LinkedErr<DiagnosticError> {
        err.map(DiagnosticError::Parser)
    }
    pub fn from_semantic_err(err: LinkedErr<SemanticError>) -> LinkedErr<DiagnosticError> {
        err.map(DiagnosticError::Semantic)
    }
    pub fn from_lint_err(err: LinkedErr<LintError>) -> LinkedErr<DiagnosticError> {
        err.map(DiagnosticError::Lint)
    }
}

impl ErrorCode for DiagnosticError {
    fn code(&self) -> &'static str {
        match self {
            Self::Parser(err) => err.code(),
            Self::Semantic(err) => err.code(),
            Self::Lint(err) => err.code(),
        }
    }

    fn src(&self) -> ErrorSource {
        match self {
            Self::Parser(err) => err.src(),
            Self::Semantic(err) => err.src(),
            Self::Lint(err) => err.src(),
        }
    }
}

impl fmt::Display for DiagnosticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parser(err) => write!(f, "{err}"),
            Self::Semantic(err) => write!(f, "{err}"),
            Self::Lint(err) => write!(f, "{err}"),
        }
    }
}
