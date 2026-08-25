use std::{fmt, slice};

use diagnostics::*;
use parser::*;

use crate::{ScriptError, SemanticError};

#[derive(Debug)]
pub enum DiagnosticError {
    Parser(ParserError),
    Semantic(SemanticError),
}

pub(crate) trait ConvertDiagnosticError<E> {
    fn convert(err: E) -> LinkedErr<DiagnosticError>;
}

impl ConvertDiagnosticError<LinkedErr<ParserError>> for DiagnosticError {
    fn convert(err: LinkedErr<ParserError>) -> LinkedErr<DiagnosticError> {
        LinkedErr::by_link(DiagnosticError::Parser(err.e), err.link)
    }
}

impl ConvertDiagnosticError<LinkedErr<SemanticError>> for DiagnosticError {
    fn convert(err: LinkedErr<SemanticError>) -> LinkedErr<DiagnosticError> {
        LinkedErr::by_link(DiagnosticError::Semantic(err.e), err.link)
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

#[derive(Debug)]
pub struct ScriptDiagnostics {
    parser: Parser,
    diagnostics: Vec<LinkedErr<DiagnosticError>>,
}

impl ScriptDiagnostics {
    pub(crate) fn new(parser: Parser, diagnostics: Vec<LinkedErr<DiagnosticError>>) -> Self {
        Self {
            parser,
            diagnostics,
        }
    }
}

pub struct LinkedDiagnostic<'a> {
    parser: &'a Parser,
    pub err: &'a LinkedErr<DiagnosticError>,
}

impl<'a> LinkedDiagnostic<'a> {
    pub fn report(&self) -> Result<String, ScriptError> {
        Ok(self.parser.report_err(self.err)?)
    }

    pub fn inner(&self) -> &'a LinkedErr<DiagnosticError> {
        self.err
    }
}

impl<'a> fmt::Display for LinkedDiagnostic<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err.e)
    }
}

pub struct ScriptDiagnosticsIter<'a> {
    parser: &'a Parser,
    diagnostics: slice::Iter<'a, LinkedErr<DiagnosticError>>,
}

impl<'a> Iterator for ScriptDiagnosticsIter<'a> {
    type Item = LinkedDiagnostic<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(LinkedDiagnostic {
            parser: self.parser,
            err: self.diagnostics.next()?,
        })
    }
}

impl fmt::Display for ScriptDiagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for diagnostic in self {
            writeln!(f, "{}", diagnostic.report().map_err(|_| fmt::Error)?)?;
        }
        Ok(())
    }
}

impl<'a> IntoIterator for &'a ScriptDiagnostics {
    type Item = LinkedDiagnostic<'a>;
    type IntoIter = ScriptDiagnosticsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ScriptDiagnosticsIter {
            parser: &self.parser,
            diagnostics: self.diagnostics.iter(),
        }
    }
}
