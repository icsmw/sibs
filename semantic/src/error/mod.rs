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
    #[error("Cannot find not assigned type, not annotated type")]
    MissedAssignedAndAnnotatedType,
    #[error("Attempt to leave global scope")]
    AttemptToLeaveGlobalScope,
    #[error("Attempt to set type without scope")]
    NoCurrentScope,
    #[error("If statement doesn't have any blocks")]
    InvalidIfStatement,
    #[error("Variable isn't defined")]
    VariableIsNotDefined,
    #[error("Nagation condition can be used only with bool types")]
    NegationToNotBool,
    #[error("Unexpected node: {0}")]
    UnexpectedNode(NodeId),
    #[error("Empty type declaration")]
    EmptyTypeDeclaration,
    #[error("Expected bool type, but actual type is: {0}")]
    ExpectedBoolType(DataType),
    #[error("Expected numeric type, but actual type is: {0}")]
    ExpectedNumericType(DataType),
    #[error("Accessor can be used only on parent value")]
    AccessorWithoutParent,
    #[error("Accessor cannot be used with: {0}")]
    AccessorOnWrongType(DataType),
}
