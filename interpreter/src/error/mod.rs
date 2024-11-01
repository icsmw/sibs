use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Next nodes are in conflict: {0}")]
    NodesAreInConflict(String),
}
