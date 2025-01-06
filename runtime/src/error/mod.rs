use crate::*;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Attempt to leave global scope")]
    AttemptToLeaveGlobalScope,
    #[error("Attempt to set type without scope")]
    NoCurrentScope,
    #[error("Variable \"{0}\" isn't found")]
    ScopeNotFound(Uuid),
    #[error("Fail to receive message")]
    RecvError,
    #[error("Fail to send message")]
    SendError,
    #[error("Fail extract value")]
    FailExtractValue,
    #[error("Invalid value type; expected \"{0}\"")]
    InvalidValueType(String),
    #[error("Value type cannot be cast to public Ty")]
    NotPublicValueType,
    #[error("Some values cannot be converted into string")]
    CannotBeConvertedToString,
    #[error("Missed binary operator")]
    MissedBinaryOperator,
    #[error("Value cannot be compared")]
    NotComparableValue,
    #[error("Values cannot be compared, because of different type")]
    DifferentTypeOfValues,
    #[error("Invalid ComparisonSeq; cannot get the first value")]
    InvalidComparisonSeq,
    #[error("Invalid If statement; cannot get the final condition")]
    InvalidIfStatement,
    #[error("Fail to infer type")]
    FailInferType,
    #[error("Unexpected node: {0}")]
    UnexpectedNode(NodeId),
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("Attempt to leave root scope's level")]
    AttemptToLeaveRootScopeLevel,
    #[error("Attempt to set type without root scope's level")]
    NoCurrentScopeLevel,
    #[error("No root scope found")]
    NoRootScope,
    #[error("Fail to find scope {0}")]
    FailToFindScope(Uuid),
    #[error("Cannot convert \"{0}\" into \"{0}\"")]
    FailCovertToRsType(String, String),

    #[error("Function \"{0}\" has been registred already")]
    FuncAlreadyRegistered(String),
    #[error("Closure \"{0}\" has been registred already")]
    ClosureAlreadyRegistered(Uuid),
    #[error("Function \"{0}\" not found")]
    FuncNotFound(String),
    #[error("Closure \"{0}\" not found")]
    ClosureNotFound(Uuid),
    #[error("Invalid function argument")]
    InvalidFnArgument,
    #[error("Invalid function argument type")]
    InvalidFnArgumentType,
    #[error("Calling function on parent without value")]
    NoParentValueToCallFn,
    #[error("Function argument type dismatch; expected: {0}")]
    FnArgumentTypeDismatch(String),
    #[error("Node \"{0}\" doesn't have linked functions")]
    NoLinkedFunctions(Uuid),
    #[error("Function \"{0}\" isn't inited")]
    NotInitedFunction(String),
    #[error("Closure \"{0}\" isn't inited")]
    NotInitedClosure(Uuid),

    #[error("Component \"{0}\" doesn't exist")]
    CompNotFound(String),
    #[error("Task \"{0}\" on component \"{1}\" doesn't exist")]
    TaskNotFound(String, String),

    #[error("Function has been declared multiple arguments with type Repeated. Only one repeated argument can be defined (at the end)")]
    MultipleRepeatedFnArgsDeclared,
    #[error("Repeated argument can be defined only once at the end")]
    NotLastRepeatedFnArg,
}

impl From<oneshot::error::RecvError> for E {
    fn from(_: oneshot::error::RecvError) -> Self {
        E::RecvError
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for E {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        E::SendError
    }
}
