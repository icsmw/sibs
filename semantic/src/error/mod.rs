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
    #[error("Assignation can't be done with IndeterminateType")]
    IndeterminateType,
    #[error("Attempt to leave global scope")]
    AttemptToLeaveGlobalScope,
    #[error("Attempt to set type without scope")]
    NoCurrentScope,
    #[error("If statement doesn't have any blocks")]
    InvalidIfStatement,
    #[error("Variable isn't defined")]
    VariableIsNotDefined,
    #[error("Unexpected node: {0}")]
    UnexpectedNode(NodeId),
    #[error("Empty type declaration")]
    EmptyTypeDeclaration,
    #[error("Expected bool type, but actual type is: {0}")]
    ExpectedBoolType(DataType),
    #[error("Expected numeric type, but actual type is: {0}")]
    ExpectedNumericType(DataType),
}
