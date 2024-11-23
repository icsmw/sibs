use crate::*;
use diagnostics::*;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Next nodes are in conflict")]
    NodesAreInConflict,
}
