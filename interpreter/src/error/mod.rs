use thiserror::Error;

use crate::*;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Next nodes are in conflict: {0}")]
    NodesAreInConflict(String),
}
