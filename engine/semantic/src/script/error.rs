use diagnostics::LinkedErr;
use enum_ids::enum_ids;
use thiserror::Error;

use crate::ScriptDiagnostics;

#[derive(Error, Debug)]
#[enum_ids(derive = "Debug")]
pub enum E {
    #[error("Fail to read valid script from \"{0}\"")]
    FailExtractAnchorNodeFrom(String),

    #[error("Lexer error: {0}")]
    Lexer(lexer::LexerError),

    #[error("Parser error: {0}")]
    Parser(parser::ParserError),

    #[error("Parsing error: {0:?}")]
    Parsing(LinkedErr<parser::ParserError>),

    #[error("Semantic error: {0:?}")]
    Semantic(LinkedErr<crate::SemanticError>),

    #[error("Script has diagnostics: {0:?}")]
    Diagnostics(ScriptDiagnostics),

    #[error("IO error: {0}")]
    IO(String),

    #[error("Runtime setup error: {0}")]
    Runtime(runtime::RtError),
}

impl From<lexer::LexerError> for E {
    fn from(err: lexer::LexerError) -> Self {
        Self::Lexer(err)
    }
}

impl From<parser::ParserError> for E {
    fn from(err: parser::ParserError) -> Self {
        Self::Parser(err)
    }
}

impl From<runtime::RtError> for E {
    fn from(err: runtime::RtError) -> Self {
        Self::Runtime(err)
    }
}

impl From<std::io::Error> for E {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err.to_string())
    }
}
