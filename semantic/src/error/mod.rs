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
    #[error("Variable \"{0}\" isn't defined")]
    VariableIsNotDefined(String),
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
    #[error("Call bound function can be used only on parent value")]
    CallWithoutParent,
    #[error("Cannot find fn call ast-node")]
    NoFnCallNodeFound,
    #[error("Accessor cannot be used with: {0}")]
    AccessorOnWrongType(DataType),
    #[error("Function \"{0}\" already exists")]
    FuncExists(String),
    #[error("Invalid module name; cannot recognize")]
    InvalidModuleName,
    #[error("Invalid function name; cannot recognize")]
    InvalidFnName,
    #[error("Fail to declare fn; error:{0}")]
    FnDeclarationError(String),
    #[error("Function \"{0}\" not found")]
    FnNotFound(String),
    #[error("Function \"{0}\" expect {1} arguments; got: {2}")]
    FnArgsNumberDismatch(String, usize, usize),
    #[error("Fail to infer type of function \"{0}\"")]
    FailInferFnResultType(String),
}
