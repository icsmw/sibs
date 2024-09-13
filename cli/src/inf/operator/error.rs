use crate::{
    error::LinkedErr,
    functions,
    inf::{
        context::{self, atlas},
        scenario, spawner, tracker, TokenGetter, ValueRef,
    },
    reader,
};
use thiserror::Error;
use tokio::sync::oneshot;
use uuid::Uuid;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Channel error: {0}")]
    Channel(String),
    #[error("Invalid variable declaration")]
    InvalidVariableDeclaration,
    #[error("Attempt to create reference from none string arguments")]
    NoneStringTaskArgumentForReference,
    #[error("Cannot extract variable name")]
    NoVariableName,
    #[error("Variable \"{0}\" isn't declared")]
    VariableIsNotDeclared(String),
    #[error("Component {0} hasn't been found")]
    UnknownComponent(Uuid),
    #[error("Type dismatch: {0} and {1}")]
    DismatchTypes(ValueRef, ValueRef),
    #[error("Gatekeeper doesn't return bool value")]
    NoBoolValueFromGatekeeper,
    #[error("Spawned process exit with error")]
    SpawnedProcessExitWithError,
    #[error("Different parts/threads returns different types")]
    ReturnsDifferentTypes,
    #[error("Element doesn't have return type")]
    NoReturnType,
    #[error("{0}")]
    TrackerError(tracker::E),
    #[error("Join error: {0}")]
    JoinError(String),
    #[error("{0}")]
    SpawningError(spawner::E),
    #[error("{0}")]
    ContextError(context::E),
    #[error("Expecting numeric value")]
    ExpectedNumericValue,
    #[error("{0}")]
    ExecutorError(functions::E),
    #[error("{0}")]
    ReaderError(reader::E),
    #[error("No task for component: {0}. Available tasks: {1:?}")]
    NoTaskForComponent(String, Vec<String>),
    #[error("Task \"{1}\" doesn't exist on component \"{0}\". Available tasks: {2:?}")]
    TaskNotExists(String, String, Vec<String>),
    #[error("Fail to extract value")]
    FailToExtractValue,
    #[error("Fail to extract accessor index")]
    FailToExtractAccessorIndex,
    #[error("Accessor index has negative value: {0}")]
    NegativeAccessorIndex(isize),
    #[error("Fail to get expected argument")]
    NoExpectedArgument,
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
    #[error("Block element expected")]
    BlockElementExpected,
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
    #[error("Variable \"{0}\" isn't assigned")]
    VariableIsNotAssigned(String),
    #[error("Fail assign variable \"{0}\"; no value")]
    NoValueToAssign(String),
    #[error("Fail to convert input for each statements into vector of strings")]
    FailConvertInputIntoStringsForEach,
    #[error("Invalid target for \"for\" statement")]
    InvalidTargetForStatement,
    #[error("Invalid range for \"for\" statement")]
    InvalidRangeForStatement,
    #[error("Invalid index variable for \"for\" statement")]
    InvalidIndexVariableForStatement,
    #[error("Attempt to get access to Metadata out of Element's scope: {0}")]
    AttemptToGetMetadataOutOfElement(String),
    #[error("Declared {0} argument(s) ([{1}]); passed {2} argument(s) ([{3}])")]
    DismatchTaskArgumentsCount(usize, String, usize, String),
    #[error("Fail to get value for declaration task's argument")]
    NoValueToDeclareTaskArgument,
    #[error("Attempt to break block, which doesn't have a break-signal")]
    NoBreakSignalSetupForBlock,
    #[error("Element isn't Task; element type: {0}")]
    ElementIsNotTask(String),
    #[error("Element isn't Component; element type: {0}")]
    ElementIsNotComponent(String),
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
    #[error("Function \"{0}\" required {1} arguments; gotten {2} arguments")]
    FunctionsArgsNumberNotMatch(String, usize, usize),
    #[error("Function \"{0}\": argument type doesn't match: {1}; gotten: {2}")]
    FunctionsArgNotMatchType(String, ValueRef, ValueRef),
    #[error("Function \"{0}\" uses Repeated type not as last argument")]
    InvalidRepeatedType(String),
    #[error("Function \"{0}\" uses Repeated and Optional types together")]
    RepeatedAndOptionalTypes(String),
    #[error("Variable \"${0}\" defined/declared multiple times")]
    MultipleDeclaration(String),
    #[error("Invalid value ref: {0}")]
    InvalidValueRef(String),
    #[error("Attempt to call PPM without prev value")]
    CallPPMWithoutPrevValue,
    #[error("Accessor can be used with Values and String only; current: {0}")]
    NotSupportedTypeByAccessor(ValueRef),
    #[error("Requested element out of bounds; length of source {0}; requested index {1}")]
    OutOfBounds(usize, usize),
    #[error("Elements in vector have different type; prev = {0}; next = {1}")]
    DismatchTypesInVector(String, String),
    #[error("Empty vector on initialization")]
    EmptyVector,
    #[error("Access by index isn't supported for: {0}")]
    AccessByIndexNotSupported(String),
    #[error("{0}")]
    AtlasError(atlas::E),
    #[error("{0}")]
    ScenarioError(scenario::E),
    #[error("Fail to recv channel message: {0}")]
    Recv(String),
}

impl From<oneshot::error::RecvError> for E {
    fn from(value: oneshot::error::RecvError) -> Self {
        E::Recv(value.to_string())
    }
}

impl E {
    pub fn by(self, operator: &dyn TokenGetter) -> LinkedErr<E> {
        LinkedErr::new(self, Some(operator.token()))
    }
    pub fn linked(self, token: &usize) -> LinkedErr<E> {
        LinkedErr::new(self, Some(*token))
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

impl From<LinkedErr<context::E>> for LinkedErr<E> {
    fn from(e: LinkedErr<context::E>) -> Self {
        LinkedErr::new(e.e.into(), e.token)
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

impl From<functions::E> for E {
    fn from(e: functions::E) -> Self {
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

impl From<functions::E> for LinkedErr<E> {
    fn from(e: functions::E) -> Self {
        LinkedErr::unlinked(E::ExecutorError(e))
    }
}
