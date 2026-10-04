mod codes;

use diagnostics::LinkedErr;
use enum_ids::enum_ids;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
#[enum_ids(derive = "Debug")]
pub enum E {
    #[error("IO error: {0}")]
    IO(String),
    #[error("Diagnostics error: {0}")]
    Diagnostics(#[from] diagnostics::DiagnosticsError),

    #[error("Fail to read valid scenario from \"{0}\"")]
    FailExtractAnchorNodeFrom(String),
    #[error("Script has been executed already")]
    ScriptAlreadyExecuted,
    #[error("Detected task inside function ({0}) declaration")]
    TaskInsideFuncDeclaration(Uuid),
    #[error("Detected nested tasks (nested task's uuid {0})")]
    NestedTasks(Uuid),
    #[error("Parser error: {0}")]
    Parser(parser::ParserError),
    #[error("Lexer error: {0}")]
    Lexer(lexer::LexerError),
    #[error("Semantic error: {0}")]
    Semantic(semantic::SemanticError),
    #[error("Runtime error: {0}")]
    Runtime(runtime::RtError),
    #[error("Fail to get access to context")]
    ContextError,
    #[error("Attempt to reuse InterContext")]
    UsedContext,
    #[error("Script is not executable. See diagnostics")]
    NotExecutable,
}

impl From<std::io::Error> for E {
    fn from(err: std::io::Error) -> Self {
        E::IO(err.to_string())
    }
}

impl From<interpreter::ScriptError> for E {
    fn from(err: interpreter::ScriptError) -> Self {
        use interpreter::ScriptError;
        match err {
            ScriptError::FailExtractAnchorNodeFrom(src) => Self::FailExtractAnchorNodeFrom(src),
            ScriptError::Lexer(err) => Self::Lexer(err),
            ScriptError::Parser(err) => Self::Parser(err),
            ScriptError::ContextError => Self::ContextError,
            ScriptError::UsedContext => Self::UsedContext,
            ScriptError::NotExecutable => Self::NotExecutable,
            ScriptError::IO(err) => Self::IO(err),
            ScriptError::Runtime(err) => Self::Runtime(err),
        }
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
