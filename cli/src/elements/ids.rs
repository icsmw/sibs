use std::{fmt, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
pub enum ElementId {
    Call,
    Accessor,
    Function,
    If,
    IfCondition,
    IfSubsequence,
    IfThread,
    Each,
    Breaker,
    First,
    Join,
    VariableAssignation,
    Compute,
    Optional,
    Gatekeeper,
    Reference,
    PatternString,
    VariableName,
    Comparing,
    Combination,
    Subsequence,
    Condition,
    Values,
    Block,
    Meta,
    Command,
    Task,
    Component,
    Integer,
    Boolean,
    VariableDeclaration,
    VariableVariants,
    VariableType,
    SimpleString,
    Range,
    For,
    Return,
    Error,
    Incrementer,
    Loop,
    While,
    Closure,
    Conclusion,
    #[allow(unused)]
    Comment,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Sufficiency {
    Sufficient,
    Not,
}

pub trait UniquenessSorting {
    fn sort_by_uniqueness(&self) -> Vec<&ElementId>;
}

impl UniquenessSorting for &[ElementId] {
    fn sort_by_uniqueness(&self) -> Vec<&ElementId> {
        self.iter()
            .filter(|el| el.is_sufficient())
            .chain(self.iter().filter(|el| !el.is_sufficient()))
            .collect()
    }
}

impl ElementId {
    fn sufficiency(&self) -> Sufficiency {
        match self {
            Self::Call => Sufficiency::Sufficient,
            Self::Accessor => Sufficiency::Sufficient,
            Self::Function => Sufficiency::Not,
            Self::If => Sufficiency::Sufficient,
            Self::IfCondition => Sufficiency::Not,
            Self::IfSubsequence => Sufficiency::Not,
            Self::IfThread => Sufficiency::Not,
            Self::Each => Sufficiency::Sufficient,
            Self::Breaker => Sufficiency::Sufficient,
            Self::First => Sufficiency::Sufficient,
            Self::Join => Sufficiency::Sufficient,
            Self::VariableAssignation => Sufficiency::Sufficient,
            Self::Optional => Sufficiency::Sufficient,
            Self::Gatekeeper => Sufficiency::Sufficient,
            Self::Reference => Sufficiency::Not,
            Self::PatternString => Sufficiency::Not,
            Self::VariableName => Sufficiency::Not,
            Self::Comparing => Sufficiency::Not,
            Self::Combination => Sufficiency::Not,
            Self::Subsequence => Sufficiency::Not,
            Self::Condition => Sufficiency::Not,
            Self::Values => Sufficiency::Not,
            Self::Block => Sufficiency::Sufficient,
            Self::Meta => Sufficiency::Sufficient,
            Self::Command => Sufficiency::Not,
            Self::Task => Sufficiency::Sufficient,
            Self::Component => Sufficiency::Sufficient,
            Self::Integer => Sufficiency::Not,
            Self::Boolean => Sufficiency::Not,
            Self::VariableDeclaration => Sufficiency::Not,
            Self::VariableVariants => Sufficiency::Not,
            Self::VariableType => Sufficiency::Not,
            Self::SimpleString => Sufficiency::Not,
            Self::Range => Sufficiency::Not,
            Self::For => Sufficiency::Sufficient,
            Self::Return => Sufficiency::Sufficient,
            Self::Error => Sufficiency::Sufficient,
            Self::Compute => Sufficiency::Sufficient,
            Self::Incrementer => Sufficiency::Sufficient,
            Self::Loop => Sufficiency::Sufficient,
            Self::While => Sufficiency::Sufficient,
            Self::Closure => Sufficiency::Sufficient,
            Self::Conclusion => Sufficiency::Not,
            Self::Comment => Sufficiency::Sufficient,
        }
    }

    pub fn is_sufficient(&self) -> bool {
        matches!(self.sufficiency(), Sufficiency::Sufficient)
    }

    pub fn resolve_conflict(&self, conflicted: ElementId) -> Option<ElementId> {
        match self {
            Self::Call => None,
            Self::Accessor => None,
            Self::Function => None,
            Self::If => None,
            Self::IfCondition => {
                if matches!(conflicted, ElementId::IfSubsequence) {
                    Some(ElementId::IfSubsequence)
                } else {
                    None
                }
            }
            Self::IfSubsequence => {
                if matches!(conflicted, ElementId::IfCondition) {
                    Some(ElementId::IfSubsequence)
                } else {
                    None
                }
            }
            Self::IfThread => None,
            Self::Each => None,
            Self::Breaker => None,
            Self::First => None,
            Self::Join => None,
            Self::VariableAssignation => None,
            Self::Optional => None,
            Self::Gatekeeper => None,
            Self::Reference => None,
            Self::PatternString => None,
            Self::VariableName => None,
            Self::Comparing => None,
            Self::Combination => None,
            Self::Subsequence => None,
            Self::Condition => None,
            Self::Values => None,
            Self::Block => None,
            Self::Meta => None,
            Self::Command => None,
            Self::Task => None,
            Self::Component => None,
            Self::Integer => None,
            Self::Boolean => None,
            Self::VariableDeclaration => None,
            Self::VariableVariants => None,
            Self::VariableType => None,
            Self::SimpleString => None,
            Self::Range => None,
            Self::For => None,
            Self::Return => None,
            Self::Error => None,
            Self::Compute => None,
            Self::Incrementer => None,
            Self::Loop => None,
            Self::While => None,
            Self::Closure => None,
            Self::Conclusion => None,
            Self::Comment => None,
        }
    }
}

impl fmt::Display for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Call => "Call",
                Self::Accessor => "Accessor",
                Self::Function => "Function",
                Self::If => "If",
                Self::IfCondition => "IfCondition",
                Self::IfSubsequence => "IfSubsequence",
                Self::IfThread => "IfThread",
                Self::Each => "Each",
                Self::Breaker => "Breaker",
                Self::First => "First",
                Self::Join => "Join",
                Self::VariableAssignation => "VariableAssignation",
                Self::Optional => "Optional",
                Self::Gatekeeper => "Gatekeeper",
                Self::Reference => "Reference",
                Self::PatternString => "PatternString",
                Self::VariableName => "VariableName",
                Self::Comparing => "Comparing",
                Self::Combination => "Combination",
                Self::Subsequence => "Subsequence",
                Self::Condition => "Condition",
                Self::Values => "Values",
                Self::Block => "Block",
                Self::Meta => "Meta",
                Self::Command => "Command",
                Self::Task => "Task",
                Self::Component => "Component",
                Self::Integer => "Integer",
                Self::Boolean => "Boolean",
                Self::VariableDeclaration => "VariableDeclaration",
                Self::VariableVariants => "VariableVariants",
                Self::VariableType => "VariableType",
                Self::SimpleString => "SimpleString",
                Self::Range => "Range",
                Self::For => "For",
                Self::Return => "Return",
                Self::Error => "Error",
                Self::Compute => "Compute",
                Self::Incrementer => "Incrementer",
                Self::Loop => "Loop",
                Self::While => "While",
                Self::Closure => "Closure",
                Self::Conclusion => "Conclusion",
                Self::Comment => "Comment",
            },
        )
    }
}
