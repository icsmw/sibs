use std::io;

use crate::CodeSourceError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DiagnosticsError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Code source error: {0}")]
    CodeSourceError(#[source] CodeSourceError),
    #[error("Source not found by uuid: {0}")]
    NotFound(Uuid),
}

impl From<CodeSourceError> for DiagnosticsError {
    fn from(err: CodeSourceError) -> Self {
        Self::CodeSourceError(err)
    }
}
