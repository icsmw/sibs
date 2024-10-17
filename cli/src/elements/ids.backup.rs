use std::{collections::HashMap, fmt, hash::Hash};

use crate::reader::{chars, words, E};

const MAX_DEEP: u8 = 2;
const MAX_PRIORITY: u8 = 7;

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
pub enum Leading {
    Char(char),
    Word(String),
    AnyNumber,
    AnyLetter,
    Group(char, char),
    // ex: $var =
    StartsWithAndHasAfter(char, char),
    // ex: () =>
    GroupByGroup(char, char, char, char),
    // ex: func_name(
    MixedFromLetterAndGroup(char, char),
    Inherit(Vec<ElementId>),
}

trait LeadingPriority {
    fn priority(&self) -> u8;
}

impl LeadingPriority for Leading {
    fn priority(&self) -> u8 {
        match self {
            Self::Inherit(els) => els.iter().map(|el| el.priority()).max().unwrap_or_default(),
            Self::AnyLetter => 0,
            Self::AnyNumber => 1,
            Self::MixedFromLetterAndGroup(..) => 2,
            Self::Char(..) => 3,
            Self::StartsWithAndHasAfter(..) => 4,
            Self::Group(..) => 5,
            Self::GroupByGroup(..) => 6,
            Self::Word(..) => 7,
        }
    }
}

impl LeadingPriority for ElementId {
    fn priority(&self) -> u8 {
        self.leading()
            .iter()
            .map(|ld| ld.priority())
            .max()
            .unwrap_or_default()
    }
}

pub trait ElementLeading {
    fn sorted_by_leading(&self) -> Vec<&ElementId>;
    fn verify(&self) -> Result<(), E>;
}

impl ElementLeading for &[ElementId] {
    fn sorted_by_leading(&self) -> Vec<&ElementId> {
        let mut sorted = Vec::new();
        for pr in (0u8..=MAX_PRIORITY).rev() {
            for el in self.iter() {
                if el.priority() == pr {
                    sorted.push(el);
                }
            }
        }
        sorted
    }
    fn verify(&self) -> Result<(), E> {
        let mut map: HashMap<ElementId, Vec<Leading>> = HashMap::new();
        for el in self.iter() {
            let required = el.leading();
            for ld in required.iter() {
                if let Some((prev_el, _)) = map.iter().find(|(_, lds)| lds.contains(ld)) {
                    return Err(E::ElementsLeadingInConflict(*el, ld.clone(), *prev_el));
                }
            }
            map.insert(*el, required);
        }
        Ok(())
    }
}

impl ElementId {
    fn leading(&self) -> Vec<Leading> {
        match self {
            Self::Call => vec![Leading::Char(chars::DOT)],
            Self::Accessor => vec![Leading::Char(chars::OPEN_SQ_BRACKET)],
            Self::Function => vec![Leading::MixedFromLetterAndGroup(
                chars::OPEN_BRACKET,
                chars::CLOSE_BRACKET,
            )],
            Self::If => vec![Leading::Word(words::IF.to_owned())],
            Self::IfCondition => vec![Leading::Group(chars::OPEN_BRACKET, chars::CLOSE_BRACKET)],
            Self::IfSubsequence => vec![Leading::Inherit(vec![
                ElementId::Boolean,
                ElementId::Command,
                ElementId::Comparing,
                ElementId::Function,
                ElementId::VariableName,
                ElementId::Reference,
                ElementId::IfCondition,
            ])],
            Self::IfThread => vec![
                Leading::Word(words::IF.to_owned()),
                Leading::Word(words::ELSE.to_owned()),
            ],
            Self::Each => vec![Leading::Word(words::EACH.to_owned())],
            Self::Breaker => vec![Leading::Word(words::BREAK.to_owned())],
            Self::First => vec![Leading::Word(words::FIRST.to_owned())],
            Self::Join => vec![Leading::Word(words::JOIN.to_owned())],
            Self::VariableAssignation => {
                vec![Leading::StartsWithAndHasAfter(chars::DOLLAR, chars::EQUAL)]
            }
            Self::Optional => vec![Leading::Inherit(vec![
                ElementId::Function,
                ElementId::VariableName,
                ElementId::Block,
                ElementId::Reference,
                ElementId::Comparing,
            ])],
            Self::Gatekeeper => vec![Leading::Inherit(vec![ElementId::Function])],
            Self::Reference => vec![Leading::Char(chars::COLON)],
            Self::PatternString => vec![Leading::Char(chars::QUOTES)],
            Self::VariableName => vec![Leading::Char(chars::DOLLAR)],
            Self::Comparing => vec![Leading::Inherit(vec![
                ElementId::VariableName,
                ElementId::Command,
                ElementId::Function,
                ElementId::PatternString,
                ElementId::Integer,
                ElementId::Boolean,
            ])],
            Self::Combination => vec![
                Leading::Word(words::AND.to_owned()),
                Leading::Word(words::OR.to_owned()),
            ],
            Self::Subsequence => vec![Leading::Inherit(vec![ElementId::Comparing])],
            Self::Condition => vec![Leading::Group(chars::OPEN_BRACKET, chars::CLOSE_BRACKET)],
            Self::Values => vec![Leading::Group(chars::OPEN_BRACKET, chars::CLOSE_BRACKET)],
            Self::Block => vec![Leading::Group(
                chars::OPEN_CURLY_BRACE,
                chars::CLOSE_CURLY_BRACE,
            )],
            Self::Meta => vec![Leading::Word(words::META.to_owned())],
            Self::Command => vec![Leading::Char(chars::TILDA)],
            Self::Task => vec![Leading::Char(chars::AT)],
            Self::Component => vec![Leading::Char(chars::POUND_SIGN)],
            Self::Integer => vec![Leading::AnyNumber],
            Self::Boolean => vec![
                Leading::Word(words::TRUE.to_owned()),
                Leading::Word(words::FALSE.to_owned()),
            ],
            Self::VariableDeclaration => vec![Leading::Inherit(vec![ElementId::VariableName])],
            Self::VariableVariants => vec![Leading::AnyLetter],
            Self::VariableType => vec![Leading::Group(
                chars::OPEN_CURLY_BRACE,
                chars::CLOSE_CURLY_BRACE,
            )],
            Self::SimpleString => vec![Leading::AnyLetter],
            Self::Range => vec![Leading::Char(chars::DOLLAR), Leading::AnyNumber],
            Self::For => vec![Leading::Word(words::FOR.to_owned())],
            Self::Return => vec![Leading::Word(words::RETURN.to_owned())],
            Self::Error => vec![Leading::Word(words::ERROR.to_owned())],
            Self::Compute => vec![Leading::Inherit(vec![
                ElementId::VariableName,
                ElementId::Function,
                ElementId::If,
                ElementId::Block,
                ElementId::Integer,
            ])],
            Self::Incrementer => vec![Leading::Inherit(vec![ElementId::VariableName])],
            Self::Loop => vec![Leading::Word(words::LOOP.to_owned())],
            Self::While => vec![Leading::Word(words::WHILE.to_owned())],
            Self::Closure => vec![Leading::GroupByGroup(
                chars::OPEN_BRACKET,
                chars::CLOSE_BRACKET,
                chars::OPEN_CURLY_BRACE,
                chars::CLOSE_CURLY_BRACE,
            )],
            Self::Conclusion => vec![Leading::Inherit(vec![
                ElementId::Comparing,
                ElementId::Condition,
            ])],
            Self::Comment => vec![Leading::Word(words::COMMENT.to_owned())],
        }
    }

    fn leading_flatten(&self, deep: u8) -> Result<Vec<Leading>, E> {
        if deep > MAX_DEEP {
            return Err(E::MaxLevelOfInheriting);
        }
        let leads = self.leading();
        let mut flatten = Vec::new();
        for ld in leads {
            if let Leading::Inherit(els) = ld {
                for el in els.iter() {
                    flatten = [flatten, el.leading_flatten(deep + 1)?].concat();
                }
            } else {
                flatten.push(ld);
            }
        }
        flatten.sort();
        flatten.dedup();
        Ok(flatten)
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
