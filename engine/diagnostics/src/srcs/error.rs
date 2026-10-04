use std::{io, path::PathBuf};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CodeSourceError {
    #[error("Source IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Cyclic file import: {}", path.display())]
    ImportCycle { path: PathBuf, ancestry: Vec<Uuid> },
    #[error("File already imported: {0}")]
    AlreadyImported(PathBuf),
}
