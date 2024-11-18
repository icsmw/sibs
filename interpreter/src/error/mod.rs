use crate::*;
use lexer::{KindId, SrcLink, Token};
use std::fmt;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct LinkedErr<T: Clone + fmt::Display> {
    pub link: SrcLink,
    pub e: T,
}

impl<T: Clone + fmt::Display> LinkedErr<T> {
    pub fn token(err: T, token: &Token) -> Self {
        Self {
            link: token.into(),
            e: err,
        }
    }
    pub fn between(err: T, from: &Token, to: &Token) -> Self {
        Self {
            link: (from, to).into(),
            e: err,
        }
    }
    pub fn current(err: T, parser: &Parser) -> Self {
        Self {
            link: parser
                .current()
                .map(|tk| tk.into())
                .unwrap_or(SrcLink::new(0, 0, &parser.src)),
            e: err,
        }
    }
    pub fn from_current(err: T, parser: &Parser) -> Self {
        Self {
            link: parser
                .from_current()
                .map(|tks| tks.into())
                .unwrap_or(SrcLink::new(0, 0, &parser.src)),
            e: err,
        }
    }
    pub fn by_link(err: T, link: &SrcLink) -> Self {
        Self {
            link: link.to_owned(),
            e: err,
        }
    }
}

#[derive(Error, Debug, Clone)]
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
    #[error("Missed binary operator -, +, *, /")]
    MissedBinaryOperator,
    #[error("Missed comma")]
    MissedComma,
    #[error("Missed semicolon")]
    MissedSemicolon,
    #[error("Infinite number cannot be used")]
    InfiniteNumber,
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

    /// While
    #[error("In while statement comparison is missed")]
    MissedComparisonInWhile,

    /// Optional
    #[error("In optional statement action is missed")]
    MissedActionInOptional,

    /// VariableDeclaration
    #[error("Expecting variable assignation after let")]
    MissedVariableDefinition,
    #[error("Expecting variable type definition")]
    MissedVariableTypeDefinition,

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

    //FunctionDeclaration
    #[error("Missed function name")]
    MissedFnName,
    #[error("Missed function body")]
    MissedFnBlock,
    #[error("Missed function argument")]
    MissedFnArguments,

    /// Module
    #[error("Missed path to module")]
    MissedModulePath,

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
}

impl E {
    pub fn link_with_token(self, token: &Token) -> LinkedErr<E> {
        LinkedErr::token(self, token)
    }
    pub fn link_between(self, from: &Token, to: &Token) -> LinkedErr<E> {
        LinkedErr::between(self, from, to)
    }
    pub fn link_by_current(self, parser: &Parser) -> LinkedErr<E> {
        LinkedErr::current(self, parser)
    }
    pub fn link_from_current(self, parser: &Parser) -> LinkedErr<E> {
        LinkedErr::from_current(self, parser)
    }
    pub fn link(self, link: &SrcLink) -> LinkedErr<E> {
        LinkedErr::by_link(self, link)
    }
}
