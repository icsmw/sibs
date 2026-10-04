use lexer::SrcLink;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GlobalError {
    #[error("{name} (previous declaration at {previous:?})")]
    Conflict { name: String, previous: SrcLink },
}
