use crate::{
    error::LinkedErr,
    executors,
    inf::{context, context::atlas, scenario, spawner, tracker, Operator},
    reader,
};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Invalid variable declaration")]
    InvalidVariableDeclaration,
    #[error("Operator method isn't supported")]
    NotSupported,
    #[error("Function's argument doesn't have return value")]
    NotAllArguamentsHasReturn,
    #[error("Spawned process exit with error")]
    SpawnedProcessExitWithError,
    #[error("No current working folder")]
    NoCurrentWorkingFolder,
    #[error("Tracker error: {0}")]
    TrackerError(tracker::E),
    #[error("Join error: {0}")]
    JoinError(String),
    #[error("Spawing process error: {0}")]
    SpawningError(spawner::E),
    #[error("Context error: {0}")]
    ContextError(context::E),
    #[error("Executor error: {0}")]
    ExecutorError(executors::E),
    #[error("Reader error: {0}")]
    ReaderError(reader::E),
    #[error("No task for component: {0}. Available tasks: {1:?}")]
    NoTaskForComponent(String, Vec<String>),
    #[error("Task \"{1}\" doesn't exist on component \"{0}\". Available tasks: {2:?}")]
    TaskNotExists(String, String, Vec<String>),
    #[error("Fail to extract value")]
    FailToExtractValue,
    #[error("Fail to get declared variable")]
    FailToGetDeclaredVariable,
    #[error("Cannot declare input because invalid number of income arguments")]
    InvalidNumberOfArgumentsForDeclaration,
    #[error("Fail to extract bool value for condition")]
    FailToExtractConditionValue,
    #[error("Cannot apply inverting \"!\" to empty return")]
    InvertingOnEmptyReturn,
    #[error("Cannot apply inverting \"!\" on none-bool return")]
    InvertingOnNotBool,
    #[error("Fail to get value as string")]
    FailToGetValueAsString,
    #[error("Fail to get string value")]
    FailToGetStringValue,
    #[error("Fail to get integer value")]
    FailToGetIntegerValue,
    #[error("Fail to get any value for task's argument")]
    FailToGetAnyValueAsTaskArg,
    #[error("If=proviso doesn't return bool result")]
    NoBoolResultFromProviso,
    #[error("Left side of comparing statement doesn't return result")]
    NoResultFromLeftOnComparing,
    #[error("Right side of comparing statement doesn't return result")]
    NoResultFromRightOnComparing,
    #[error("If=proviso doesn't return any result")]
    NoResultFromProviso,
    #[error("Variable \"{0}\" isn't assigned")]
    VariableIsNotAssigned(String),
    #[error("Fail assign variable \"{0}\"; no value")]
    NoValueToAssign(String),
    #[error("Fail to get input for EACH statements")]
    NoInputForEach,
    #[error("Fail to convert input for EACH statements into vector of strings")]
    FailConvertInputIntoStringsForEach,
    #[error("Declared {0} argument(s) ([{1}]); passed {2} argument(s) ([{3}])")]
    DismatchTaskArgumentsCount(usize, String, usize, String),
    #[error("Fail to get value for declaration task's argument")]
    NoValueToDeclareTaskArgument,
    #[error("Value \"{0}\" doesn't match to allowed: {1}")]
    NotDeclaredValueAsArgument(String, String),
    #[error("Reference has invalid number of parts")]
    InvalidPartsInReference,
    #[error("Owner component isn't defined")]
    NoOwnerComponent,
    #[error("Fail to find component \"{0}\"")]
    NotFoundComponent(String),
    #[error("Task \"{0}\" for component \"{1}\" not found")]
    TaskNotFound(String, String),
    #[error("Fail to parse string to {{{0}}}: {1}")]
    ParseStringError(String, String),
    #[error("Atlas error: {0}")]
    AtlasError(atlas::E),
    #[error("Scenario error: {0}")]
    ScenarioError(scenario::E),
}

impl E {
    pub fn by(self, operator: &dyn Operator) -> LinkedErr<E> {
        LinkedErr::new(self, Some(operator.token()))
    }
    pub fn unlinked(self) -> LinkedErr<E> {
        LinkedErr::new(self, None)
    }
}

impl From<tracker::E> for E {
    fn from(e: tracker::E) -> Self {
        Self::TrackerError(e)
    }
}

impl From<atlas::E> for E {
    fn from(e: atlas::E) -> Self {
        Self::AtlasError(e)
    }
}

impl From<atlas::E> for LinkedErr<E> {
    fn from(e: atlas::E) -> Self {
        LinkedErr::unlinked(E::AtlasError(e))
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

impl From<LinkedErr<reader::error::E>> for LinkedErr<E> {
    fn from(e: LinkedErr<reader::error::E>) -> Self {
        LinkedErr::new(E::ReaderError(e.e), e.token)
    }
}

impl From<reader::error::E> for LinkedErr<E> {
    fn from(e: reader::error::E) -> Self {
        LinkedErr::unlinked(E::ReaderError(e))
    }
}

impl From<scenario::E> for LinkedErr<E> {
    fn from(e: scenario::E) -> Self {
        LinkedErr::unlinked(E::ScenarioError(e))
    }
}

impl From<context::E> for LinkedErr<E> {
    fn from(e: context::E) -> Self {
        LinkedErr::unlinked(E::ContextError(e))
    }
}

impl From<tracker::E> for LinkedErr<E> {
    fn from(e: tracker::E) -> Self {
        LinkedErr::unlinked(E::TrackerError(e))
    }
}

impl From<E> for LinkedErr<E> {
    fn from(e: E) -> Self {
        LinkedErr::unlinked(e)
    }
}

impl From<LinkedErr<E>> for E {
    fn from(e: LinkedErr<E>) -> Self {
        e.e
    }
}

impl From<spawner::E> for LinkedErr<E> {
    fn from(e: spawner::E) -> Self {
        LinkedErr::unlinked(E::SpawningError(e))
    }
}

impl From<executors::E> for LinkedErr<E> {
    fn from(e: executors::E) -> Self {
        LinkedErr::unlinked(E::ExecutorError(e))
    }
}
