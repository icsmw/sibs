use diagnostics::LinkedErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("IO error: {0}")]
    IO(String),

    #[error("Fail to read valid scenario from \"{0}\"")]
    FailExtractAnchorNodeFrom(String),
    #[error("Script has been executed already")]
    ScriptAlreadyExecuted,
    #[error("Parser error: {0}")]
    Parser(parser::ParserError),
    #[error("Lexer error: {0}")]
    Lexer(lexer::LexerError),
    #[error("Semantic error: {0}")]
    Semantic(semantic::SemanticError),
    #[error("Runtime error: {0}")]
    Runtime(runtime::RtError),
}

impl From<std::io::Error> for E {
    fn from(err: std::io::Error) -> Self {
        E::IO(err.to_string())
    }
}

impl From<lexer::LexerError> for E {
    fn from(err: lexer::LexerError) -> Self {
        E::Lexer(err)
    }
}

impl From<parser::ParserError> for E {
    fn from(err: parser::ParserError) -> Self {
        E::Parser(err)
    }
}

impl From<LinkedErr<parser::ParserError>> for E {
    fn from(err: LinkedErr<parser::ParserError>) -> Self {
        E::Parser(err.e)
    }
}

impl From<semantic::SemanticError> for E {
    fn from(err: semantic::SemanticError) -> Self {
        E::Semantic(err)
    }
}

impl From<LinkedErr<semantic::SemanticError>> for E {
    fn from(err: LinkedErr<semantic::SemanticError>) -> Self {
        E::Semantic(err.e)
    }
}

impl From<runtime::RtError> for E {
    fn from(err: runtime::RtError) -> Self {
        E::Runtime(err)
    }
}

impl From<LinkedErr<runtime::RtError>> for E {
    fn from(err: LinkedErr<runtime::RtError>) -> Self {
        E::Runtime(err.e)
    }
}
