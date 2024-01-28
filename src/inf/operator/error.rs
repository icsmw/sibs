use crate::{
    executors,
    inf::{context, spawner, tracker},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Operator method isn't supported")]
    NotSupported,
    #[error("Spawned process exist with error")]
    SpawnedProcessExitWithError,
    #[error("No current working folder")]
    NoCurrentWorkingFolder,
    #[error("Tracker error: {0}")]
    TrackerError(String),
    #[error("Spawing process error: {0}")]
    SpawningError(String),
    #[error("Context error: {0}")]
    ContextError(String),
    #[error("Executor error: {0}")]
    ExecutorError(String),
    #[error("No task for component: {0}")]
    NoTaskForComponent(String),
    #[error("No task \"{0}\" for component \"{1}\" doesn't exist")]
    TaskNotExists(String, String),
    #[error("Task \"{0}\" doesn't have block with actions")]
    NoTaskBlock(String),
    #[error("Fail to extract value")]
    FailToExtractValue,
    #[error("Fail to extract bool value for condition")]
    FailToExtractConditionValue,
    #[error("Fail to get value as string")]
    FailToGetValueAsString,
    #[error("Fail to get string value")]
    FailToGetStringValue,
    #[error("Function doesn't return bool result")]
    NoBoolResultFromFunction,
    #[error("If=proviso doesn't return bool result")]
    NoBoolResultFromProviso,
    #[error("If=(proviso AND/OR proviso) doesn't return bool result")]
    NoBoolResultFromProvisoGroup,
    #[error("If=proviso doesn't return any result")]
    NoResultFromProviso,
    #[error("Combination operator (AND, OR) should follow after proviso")]
    WrongConditionsOrderInIf,
    #[error("Variable \"{0}\" isn't assigned")]
    VariableIsNotAssigned(String),
    #[error("Function \"{0}\" doesn't have registred executor")]
    NoFunctionExecutor(String),
    #[error("Fail assign variable \"{0}\"; no value")]
    NoValueToAssign(String),
    #[error("Fail to get input for EACH statements")]
    NoInputForEach,
    #[error("Fail to convert input for EACH statements into vector of strings")]
    FailConvertInputIntoStringsForEach,
    #[error("Number of arguments and declarations in task aren't match")]
    DismatchTaskArgumentsCount,
    #[error("Fail to get value for declaration task's argument")]
    NoValueToDeclareTaskArgument,
}

impl From<tracker::E> for E {
    fn from(e: tracker::E) -> Self {
        Self::TrackerError(e.to_string())
    }
}

impl From<spawner::E> for E {
    fn from(e: spawner::E) -> Self {
        Self::SpawningError(e.to_string())
    }
}

impl From<context::E> for E {
    fn from(e: context::E) -> Self {
        Self::ContextError(e.to_string())
    }
}

impl From<executors::E> for E {
    fn from(e: executors::E) -> Self {
        Self::ExecutorError(e.to_string())
    }
}
