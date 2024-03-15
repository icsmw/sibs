use crate::{
    error::LinkedErr,
    executors,
    inf::{context, spawner, tracker},
    reader,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Operator method isn't supported")]
    NotSupported,
    #[error("Function's argument doesn't have return value")]
    NotAllArguamentsHasReturn,
    #[error("Spawned process exist with error")]
    SpawnedProcessExitWithError,
    #[error("No current working folder")]
    NoCurrentWorkingFolder,
    #[error("Tracker error: {0}")]
    TrackerError(tracker::E),
    #[error("Spawing process error: {0}")]
    SpawningError(spawner::E),
    #[error("Context error: {0}")]
    ContextError(context::E),
    #[error("Executor error: {0}")]
    ExecutorError(executors::E),
    #[error("Reader error: {0}")]
    ReaderError(reader::E),
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
    #[error("Fail to get integer value")]
    FailToGetIntegerValue,
    #[error("Fail to get any value for task's argument")]
    FailToGetAnyValueAsTaskArg,
    #[error("Function doesn't return bool result")]
    NoBoolResultFromFunction,
    #[error("If=proviso doesn't return bool result")]
    NoBoolResultFromProviso,
    #[error("Left side of comparing statement doesn't return result")]
    NoResultFromLeftOnComparing,
    #[error("Right side of comparing statement doesn't return result")]
    NoResultFromRightOnComparing,
    #[error("If=(proviso AND/OR proviso) doesn't return bool result")]
    NoBoolResultFromProvisoGroup,
    #[error("If=proviso doesn't return any result")]
    NoResultFromProviso,
    #[error("Combination operator (AND, OR) should follow after proviso")]
    WrongConditionsOrderInIf,
    #[error("Variable \"{0}\" isn't assigned")]
    VariableIsNotAssigned(String),
    #[error("Fail to extract value for IF statement")]
    FailExtractValueForIFStatement,
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
    #[error("Reference has invalid number of parts")]
    InvalidPartsInReference,
    #[error("Unsupported condition of IF statement")]
    UnsupportedCondition,
    #[error("Owner component isn't defined")]
    NoOwnerComponent,
    #[error("Fail to find component \"{0}\"")]
    NotFoundComponent(String),
    #[error("Task \"{0}\" for component \"{1}\" not found")]
    TaskNotFound(String, String),
}

impl From<tracker::E> for E {
    fn from(e: tracker::E) -> Self {
        Self::TrackerError(e)
    }
}

impl From<spawner::E> for E {
    fn from(e: spawner::E) -> Self {
        Self::SpawningError(e)
    }
}

impl From<context::E> for E {
    fn from(e: context::E) -> Self {
        Self::ContextError(e)
    }
}

impl From<executors::E> for E {
    fn from(e: executors::E) -> Self {
        Self::ExecutorError(e)
    }
}

impl From<reader::error::E> for E {
    fn from(e: reader::error::E) -> Self {
        Self::ReaderError(e)
    }
}

impl From<LinkedErr<reader::error::E>> for E {
    fn from(e: LinkedErr<reader::error::E>) -> Self {
        Self::ReaderError(e.e)
    }
}
