use crate::*;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Token isn't bound to known DataType")]
    TokenIsNotBoundToKnownDataType,
    #[error("No variants are defined")]
    NoVariantsAreDefined,
    #[error("Variants have different types")]
    VariantsHaveDiffTypes,
    #[error("Types are dismatch: {0}")]
    DismatchTypes(String),
    #[error("Attempt to leave global scope")]
    AttemptToLeaveGlobalScope,
    #[error("Attempt to set type without scope")]
    NoCurrentScope,
}
