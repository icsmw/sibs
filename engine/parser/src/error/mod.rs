mod codes;

use std::{
    cell::{BorrowError, BorrowMutError},
    io,
    sync::PoisonError,
};

use crate::*;
use diagnostics::*;
use enum_ids::enum_ids;
use lexer::{KindId, LexerError, Token};
use thiserror::Error;

#[derive(Error, Debug)]
#[enum_ids(derive = "Debug")]
pub enum E {
    #[error("Next nodes are in conflict: {0}")]
    NodesAreInConflict(String),
    #[error("No closing: {0}")]
    NoClosing(KindId),
    #[error("Unexpected logical operator: {0}")]
    UnexpectedLogicalOperator(KindId),
    #[error("Unexpected binary operator: {0}")]
    UnexpectedBinaryOperator(KindId),
    #[error("Missed logical operator && or ||")]
    MissedLogicalOperator,
    #[error("Missed condition argument after operator && or ||")]
    MissedConditionArgument,
    #[error("Missed binary operator -, +, *, /")]
    MissedBinaryOperator,
    #[error("Missed binary argument after operator -, +, *, /")]
    MissedBinaryArgument,
    #[error("Missed comma")]
    MissedComma,
    #[error("Missed closing vertical bar")]
    MissedClosingBar,
    #[error("Missed semicolon")]
    MissedSemicolon,
    #[error("Infinite number cannot be used")]
    InfiniteNumber,
    #[error("Loop doesn't have a condition to be breaked")]
    NotBreakableLoop,
    #[error("Invalid right side of assignation: {0}")]
    InvalidAssignation(String),
    #[error("Error message is missed")]
    MissedErrorMessage,
    #[error("Unrecognized code: {0}")]
    UnrecognizedCode(String),
    #[error("Fail to parse string: {0}")]
    InvalidString(String),
    #[error("String has empty expression")]
    EmptyStringExpression,
    #[error("Not supported string injection in: {0}")]
    NotSupportedStringInjection(String),
    #[error("After {0} expected block")]
    NoExpectedBlockAfter(KindId),
    #[error("After {0} expected {1}, but not found")]
    MissedExpectation(String, String),
    #[error("Expected block, but it's missed")]
    MissedBlock,
    #[error("Fail find node {0}")]
    FailFindNode(Uuid),
    #[error("Probably parse has been modified in middle of parsing")]
    UnexpectedEmptyParser,
    #[error("File \"{0}\" not found")]
    FileNotFound(String),
    #[error("Exptected type: {0}; but actual is: {1}")]
    UnexpectedType(String, String),
    #[error("Parent path isn't available; using \"include from ...\" and \"mod from ...\" isn't possible")]
    NoParentPath,
    #[error("File reading error: {0:?}")]
    FileReading(#[from] std::io::Error),
    #[error("Fail to find \"{0}\" in attached content")]
    FailToFindNode(String),
    #[error("IO Error: {0}")]
    IOError(io::Error),
    #[error("Fail to get module name from {0};")]
    FailGetModuleName(String),
    #[error("Fail to get access to locked data")]
    PoisonError,
    #[error("Fail to get write access to locked data")]
    BorrowMutError,
    #[error("Fail to get read access to locked data")]
    BorrowError,

    /// Call
    #[error("Missed expression after dot")]
    MissedCallExpression,

    /// Each
    #[error("In each statement declaration of element variable is missed")]
    MissedElementDeclarationInEach,
    #[error("In each statement declaration of index variable is missed")]
    MissedIndexDeclarationInEach,
    #[error("Fail to recognize elements for each statement: {0}")]
    FailRecognizeElementsInEach(String),

    /// For
    #[error("In for statement declaration of element variable is missed")]
    MissedElementDeclarationInFor,
    #[error("In for statement declaration of index variable is missed")]
    MissedIndexDeclarationInFor,
    #[error("Fail to recognize elements for for statement: {0}")]
    FailRecognizeElementsInFor(String),
    #[error("Invalid \"for\" syntax. Ex: for el in els {{...}}; for (el, n) in els {{...}};")]
    InvalidForSyntax,
    #[error("In for statement keyword \"in\" is missed")]
    MissedInKeywordInFor,

    /// While
    #[error("In while statement comparison is missed")]
    MissedComparisonInWhile,

    /// Optional
    #[error("In optional statement action is missed")]
    MissedActionInOptional,

    /// VariableDeclaration
    #[error("Expecting variable assignation after let")]
    MissedVariableDefinition,
    #[error("Expecting variable name after let")]
    MissedVariableName,
    #[error("Expecting variable type definition")]
    MissedVariableTypeDefinition,

    #[error("Keyword cannot be used here")]
    KeywordUsing,

    /// VariableType
    #[error("Expecting variable nested type definition")]
    MissedNestedTypeDefinition,
    #[error("Unknown type: {0}")]
    UnknownType(String),

    /// ArgumentDeclaration
    #[error("Expecting argument type definition")]
    MissedArgumentTypeDefinition,

    /// Closure
    #[error("Missed closure body")]
    MissedClosureBlock,
    #[error("Missed closure return type")]
    MissedClosureReturnType,

    /// FunctionDeclaration
    #[error("Missed function name")]
    MissedFnName,
    #[error("Missed function body")]
    MissedFnBlock,
    #[error("Missed function argument")]
    MissedFnArguments,

    /// Module
    #[error("Missed path to module")]
    MissedModulePath,
    #[error("Missed body of module")]
    MissedModuleBody,

    /// Task
    #[error("Keyword private can be used only on task declaration")]
    InvalidPrivateKeyUsage,
    #[error("Missed task name")]
    MissedTaskName,
    #[error("Missed task body")]
    MissedTaskBlock,
    #[error("Missed task argument")]
    MissedTaskArguments,

    /// Component
    #[error("Missed component name")]
    MissedComponentName,
    #[error("Missed component body")]
    MissedComponentBlock,
    #[error("Missed component cwd")]
    MissedComponentCWD,
    #[error("No tasks in component")]
    NoTasksInComponent,

    /// Gatekeeper
    #[error("No gatekeeper directive")]
    NoGatekeeperDirective,

    /// Skip
    #[error("Skip directive without arguments")]
    NoSkipDirectiveArgs,
    #[error("Skip directive without task's arguments")]
    NoSkipDirectiveTaskArgs,
    #[error("Skip directive without function")]
    NoSkipDirectiveFuncCall,

    /// Return
    #[error("Invalid return value")]
    InvalidReturnValue,

    /// Lexer error
    #[error("Lexer error: {0}")]
    LexerError(LexerError),

    #[error("Unlinked error: no tokens, no errors")]
    Unlinked,
}

impl From<LexerError> for E {
    fn from(err: LexerError) -> Self {
        Self::LexerError(err)
    }
}

impl<T> From<PoisonError<T>> for E {
    fn from(_: PoisonError<T>) -> Self {
        E::PoisonError
    }
}

impl From<BorrowMutError> for E {
    fn from(_: BorrowMutError) -> Self {
        E::BorrowMutError
    }
}

impl From<BorrowError> for E {
    fn from(_: BorrowError) -> Self {
        E::BorrowError
    }
}

impl E {
    pub fn link_with_token(self, token: &Token) -> LinkedErr<E> {
        LinkedErr::token(self, token)
    }
    pub fn link_between(self, from: &Token, to: &Token) -> LinkedErr<E> {
        LinkedErr::between(self, from, to)
    }
    pub fn link_by_current(self, parser: &Parser) -> LinkedErr<E> {
        parser.err_current(self)
    }
    pub fn link_until_end(self, parser: &Parser) -> LinkedErr<E> {
        parser.err_until_end(self)
    }
    pub fn link(self, node: &LinkedNode) -> LinkedErr<E> {
        LinkedErr::from(self, node)
    }
    pub fn from(self, link: &SrcLink) -> LinkedErr<E> {
        LinkedErr {
            e: self,
            link: link.into(),
        }
    }
}
