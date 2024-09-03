mod block;
mod comment;
mod component;
mod conditions;
mod function;
mod gatekeeper;
mod meta;
mod optional;
mod ppm;
mod primitives;
mod reference;
mod statements;
mod string;
mod task;
mod values;
mod variable;

pub use block::*;
pub use comment::*;
pub use component::*;
pub use conditions::*;
pub use function::*;
pub use gatekeeper::*;
pub use meta::*;
pub use optional::*;
pub use ppm::*;
pub use primitives::*;
pub use reference::*;
pub use statements::{breaker::*, each::*, first::*, join::Join, If::*};
pub use string::{command::*, pattern::*, simple::*};
pub use task::*;
use tokio_util::sync::CancellationToken;
pub use values::*;
pub use variable::*;

use crate::{
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, GlobalVariablesMap, LinkingResult, PrevValue, Scope,
        TokenGetter, TryExecute, Value, VerificationResult,
    },
    reader::{chars, Dissect, Reader, E},
};
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ElTarget {
    Function,
    If,
    Each,
    Breaker,
    First,
    Join,
    VariableAssignation,
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
    Ppm,
    #[allow(unused)]
    Comment,
}

impl fmt::Display for ElTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Function => "Function",
                Self::If => "If",
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
                Self::Ppm => "Ppm",
                Self::Comment => "Comment",
            },
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    // Element: Comment | Meta
    pub comments: Vec<Comment>,
    pub meta: Vec<Meta>,
    pub ppm: Option<Box<Element>>,
    pub tolerance: bool,
    pub inverting: bool,
}

impl Metadata {
    pub fn empty() -> Self {
        Metadata {
            comments: Vec::new(),
            meta: Vec::new(),
            ppm: None,
            tolerance: false,
            inverting: false,
        }
    }
    #[allow(unused)]
    pub fn comments(&self) -> &[Comment] {
        &self.comments
    }
    pub fn meta(&self) -> &[Meta] {
        &self.meta
    }
    pub fn meta_as_lines(&self) -> Vec<&str> {
        self.meta.iter().flat_map(|el| el.as_lines()).collect()
    }
    pub fn set_ppm(&mut self, el: Element) {
        self.ppm = Some(Box::new(el));
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.comments
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            if self.comments.is_empty() { "" } else { "\n" },
            self.meta
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            if self.meta.is_empty() { "" } else { "\n" },
        )
    }
}

impl Formation for Metadata {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}{}{}",
            self.comments
                .iter()
                .map(|c| c.format(cursor))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.comments.is_empty() { "" } else { "\n" },
            self.meta
                .iter()
                .map(|c| c.format(cursor))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.meta.is_empty() { "" } else { "\n" },
        )
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    Function(Function, Metadata),
    If(If, Metadata),
    Breaker(Breaker, Metadata),
    Each(Each, Metadata),
    First(First, Metadata),
    Join(Join, Metadata),
    VariableAssignation(VariableAssignation, Metadata),
    Optional(Optional, Metadata),
    Gatekeeper(Gatekeeper, Metadata),
    Reference(Reference, Metadata),
    PatternString(PatternString, Metadata),
    VariableName(VariableName, Metadata),
    Comparing(Comparing, Metadata),
    Combination(Combination, Metadata),
    Subsequence(Subsequence, Metadata),
    Condition(Condition, Metadata),
    Values(Values, Metadata),
    Block(Block, Metadata),
    Command(Command, Metadata),
    Task(Task, Metadata),
    Component(Component, Metadata),
    Boolean(Boolean, Metadata),
    Integer(Integer, Metadata),
    VariableDeclaration(VariableDeclaration, Metadata),
    VariableVariants(VariableVariants, Metadata),
    VariableType(VariableType, Metadata),
    SimpleString(SimpleString, Metadata),
    Ppm(Ppm, Metadata),
    Meta(Meta),
    Comment(Comment),
}

impl Element {
    fn parse(
        reader: &mut Reader,
        targets: &[ElTarget],
        includes: bool,
    ) -> Result<Option<Element>, LinkedErr<E>> {
        fn tolerance(reader: &mut Reader, mut md: Metadata) -> Metadata {
            md.tolerance = reader.move_to().char(&[&chars::QUESTION]).is_some();
            md
        }
        fn next(reader: &mut Reader, mut el: Element) -> Result<Option<Element>, LinkedErr<E>> {
            let Some(ppm) = Element::include(reader, &[ElTarget::Ppm])? else {
                return Ok(Some(el));
            };
            el.get_mut_metadata().set_ppm(ppm);
            Ok(Some(el))
        }
        let mut comments: Vec<Comment> = Vec::new();
        let mut meta: Vec<Meta> = Vec::new();
        loop {
            if let Some(el) = Comment::dissect(reader)? {
                comments.push(el);
                continue;
            }
            if let Some(el) = Meta::dissect(reader)? {
                meta.push(el);
                continue;
            }
            break;
        }
        let md = Metadata {
            comments,
            meta,
            ppm: None,
            tolerance: false,
            inverting: if targets.contains(&ElTarget::Function) {
                reader.move_to().char(&[&chars::EXCLAMATION]).is_some()
            } else {
                false
            },
        };
        if includes == targets.contains(&ElTarget::Breaker) {
            if let Some(el) = Breaker::dissect(reader)? {
                return next(reader, Element::Breaker(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Ppm) {
            if let Some(el) = Ppm::dissect(reader)? {
                return next(reader, Element::Ppm(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Combination) {
            if let Some(el) = Combination::dissect(reader)? {
                return next(reader, Element::Combination(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Subsequence) {
            if let Some(el) = Subsequence::dissect(reader)? {
                return next(reader, Element::Subsequence(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Condition) {
            if let Some(el) = Condition::dissect(reader)? {
                return next(reader, Element::Condition(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Meta) {
            if let Some(el) = Meta::dissect(reader)? {
                return next(reader, Element::Meta(el));
            }
        }
        if includes == targets.contains(&ElTarget::Command) {
            if let Some(el) = Command::dissect(reader)? {
                let to = tolerance(reader, md);
                return next(reader, Element::Command(el, to));
            }
        }
        if includes == targets.contains(&ElTarget::If) {
            if let Some(el) = If::dissect(reader)? {
                return next(reader, Element::If(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Optional) {
            if let Some(el) = Optional::dissect(reader)? {
                return next(reader, Element::Optional(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Gatekeeper) {
            if let Some(el) = Gatekeeper::dissect(reader)? {
                return next(reader, Element::Gatekeeper(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Comparing) {
            if let Some(el) = Comparing::dissect(reader)? {
                return next(reader, Element::Comparing(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Integer) {
            if let Some(el) = Integer::dissect(reader)? {
                return next(reader, Element::Integer(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Boolean) {
            if let Some(el) = Boolean::dissect(reader)? {
                return next(reader, Element::Boolean(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::VariableAssignation) {
            if let Some(el) = VariableAssignation::dissect(reader)? {
                return next(reader, Element::VariableAssignation(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::VariableName) {
            if let Some(el) = VariableName::dissect(reader)? {
                return next(reader, Element::VariableName(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Each) {
            if let Some(el) = Each::dissect(reader)? {
                return next(reader, Element::Each(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::First) {
            if let Some(el) = First::dissect(reader)? {
                return next(reader, Element::First(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Join) {
            if let Some(el) = Join::dissect(reader)? {
                return next(reader, Element::Join(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Function) {
            if let Some(el) = Function::dissect(reader)? {
                let to = tolerance(reader, md);
                return next(reader, Element::Function(el, to));
            }
        }
        if includes == targets.contains(&ElTarget::Reference) {
            if let Some(el) = Reference::dissect(reader)? {
                return next(reader, Element::Reference(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::PatternString) {
            if let Some(el) = PatternString::dissect(reader)? {
                return next(reader, Element::PatternString(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Block) {
            if let Some(el) = Block::dissect(reader)? {
                return next(reader, Element::Block(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Values) {
            if let Some(el) = Values::dissect(reader)? {
                return next(reader, Element::Values(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Component) {
            if let Some(el) = Component::dissect(reader)? {
                return next(reader, Element::Component(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::Task) {
            if let Some(el) = Task::dissect(reader)? {
                return next(reader, Element::Task(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::VariableDeclaration) {
            if let Some(el) = VariableDeclaration::dissect(reader)? {
                return next(reader, Element::VariableDeclaration(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::VariableType) {
            if let Some(el) = VariableType::dissect(reader)? {
                return next(reader, Element::VariableType(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::VariableVariants) {
            if let Some(el) = VariableVariants::dissect(reader)? {
                return next(reader, Element::VariableVariants(el, md));
            }
        }
        if includes == targets.contains(&ElTarget::SimpleString) {
            if let Some(el) = SimpleString::dissect(reader)? {
                return next(reader, Element::SimpleString(el, md));
            }
        }
        Ok(None)
    }

    pub fn exclude(
        reader: &mut Reader,
        targets: &[ElTarget],
    ) -> Result<Option<Element>, LinkedErr<E>> {
        Self::parse(reader, targets, false)
    }

    pub fn include(
        reader: &mut Reader,
        targets: &[ElTarget],
    ) -> Result<Option<Element>, LinkedErr<E>> {
        Self::parse(reader, targets, true)
    }

    pub fn get_metadata(&self) -> &Metadata {
        match self {
            Self::Function(_, md) => md,
            Self::If(_, md) => md,
            Self::Breaker(_, md) => md,
            Self::Each(_, md) => md,
            Self::First(_, md) => md,
            Self::Join(_, md) => md,
            Self::VariableAssignation(_, md) => md,
            Self::Comparing(_, md) => md,
            Self::Combination(_, md) => md,
            Self::Condition(_, md) => md,
            Self::Subsequence(_, md) => md,
            Self::Optional(_, md) => md,
            Self::Gatekeeper(_, md) => md,
            Self::Reference(_, md) => md,
            Self::PatternString(_, md) => md,
            Self::VariableName(_, md) => md,
            Self::Values(_, md) => md,
            Self::Block(_, md) => md,
            Self::Command(_, md) => md,
            Self::Task(_, md) => md,
            Self::Component(_, md) => md,
            Self::Boolean(_, md) => md,
            Self::Integer(_, md) => md,
            Self::VariableDeclaration(_, md) => md,
            Self::VariableVariants(_, md) => md,
            Self::VariableType(_, md) => md,
            Self::SimpleString(_, md) => md,
            Self::Ppm(_, md) => md,
            Self::Comment(_) | Self::Meta(_) => {
                panic!("Comment doesn't have metadata");
            }
        }
    }

    pub fn get_mut_metadata(&mut self) -> &mut Metadata {
        match self {
            Self::Function(_, md) => md,
            Self::If(_, md) => md,
            Self::Breaker(_, md) => md,
            Self::Each(_, md) => md,
            Self::First(_, md) => md,
            Self::Join(_, md) => md,
            Self::VariableAssignation(_, md) => md,
            Self::Comparing(_, md) => md,
            Self::Combination(_, md) => md,
            Self::Condition(_, md) => md,
            Self::Subsequence(_, md) => md,
            Self::Optional(_, md) => md,
            Self::Gatekeeper(_, md) => md,
            Self::Reference(_, md) => md,
            Self::PatternString(_, md) => md,
            Self::VariableName(_, md) => md,
            Self::Values(_, md) => md,
            Self::Block(_, md) => md,
            Self::Command(_, md) => md,
            Self::Task(_, md) => md,
            Self::Component(_, md) => md,
            Self::Boolean(_, md) => md,
            Self::Integer(_, md) => md,
            Self::VariableDeclaration(_, md) => md,
            Self::VariableVariants(_, md) => md,
            Self::VariableType(_, md) => md,
            Self::SimpleString(_, md) => md,
            Self::Ppm(_, md) => md,
            Self::Comment(_) | Self::Meta(_) => {
                panic!("Comment doesn't have metadata");
            }
        }
    }

    pub fn as_task(&self) -> Result<&Task, LinkedErr<operator::E>> {
        if let Element::Task(task, _) = self {
            Ok(task)
        } else {
            Err(operator::E::ElementIsNotTask(format!("{self:?}")).linked(&self.token()))
        }
    }

    pub fn as_component(&self) -> Result<&Component, LinkedErr<operator::E>> {
        if let Element::Component(component, _) = self {
            Ok(component)
        } else {
            Err(operator::E::ElementIsNotComponent(format!("{self:?}")).linked(&self.token()))
        }
    }
    pub fn as_mut_component(&mut self) -> Result<&mut Component, LinkedErr<operator::E>> {
        if let Element::Component(component, _) = self {
            Ok(component)
        } else {
            Err(operator::E::ElementIsNotComponent(format!("{self:?}")).linked(&self.token()))
        }
    }
    #[cfg(test)]
    pub fn el_target(&self) -> ElTarget {
        match self {
            Self::Function(..) => ElTarget::Function,
            Self::If(..) => ElTarget::If,
            Self::Breaker(..) => ElTarget::Breaker,
            Self::Each(..) => ElTarget::Each,
            Self::First(..) => ElTarget::First,
            Self::Join(..) => ElTarget::Join,
            Self::VariableAssignation(..) => ElTarget::VariableAssignation,
            Self::Comparing(..) => ElTarget::Comparing,
            Self::Combination(..) => ElTarget::Combination,
            Self::Condition(..) => ElTarget::Condition,
            Self::Subsequence(..) => ElTarget::Subsequence,
            Self::Optional(..) => ElTarget::Optional,
            Self::Gatekeeper(..) => ElTarget::Gatekeeper,
            Self::Reference(..) => ElTarget::Reference,
            Self::PatternString(..) => ElTarget::PatternString,
            Self::VariableName(..) => ElTarget::VariableName,
            Self::Values(..) => ElTarget::Values,
            Self::Meta(..) => ElTarget::Meta,
            Self::Block(..) => ElTarget::Block,
            Self::Command(..) => ElTarget::Command,
            Self::Task(..) => ElTarget::Task,
            Self::Component(..) => ElTarget::Component,
            Self::Boolean(..) => ElTarget::Boolean,
            Self::Integer(..) => ElTarget::Integer,
            Self::VariableDeclaration(..) => ElTarget::VariableDeclaration,
            Self::VariableVariants(..) => ElTarget::VariableVariants,
            Self::VariableType(..) => ElTarget::VariableType,
            Self::SimpleString(..) => ElTarget::SimpleString,
            Self::Ppm(..) => ElTarget::Ppm,
            Self::Comment(..) => ElTarget::Comment,
        }
    }

    #[cfg(test)]
    pub fn inner_to_string(&self) -> String {
        match self {
            Self::Function(v, _) => v.to_string(),
            Self::If(v, _) => v.to_string(),
            Self::Each(v, _) => v.to_string(),
            Self::Breaker(v, _) => v.to_string(),
            Self::First(v, _) => v.to_string(),
            Self::Join(v, _) => v.to_string(),
            Self::VariableAssignation(v, _) => v.to_string(),
            Self::Comparing(v, _) => v.to_string(),
            Self::Combination(v, _) => v.to_string(),
            Self::Condition(v, _) => v.to_string(),
            Self::Subsequence(v, _) => v.to_string(),
            Self::Optional(v, _) => v.to_string(),
            Self::Gatekeeper(v, _) => v.to_string(),
            Self::Reference(v, _) => v.to_string(),
            Self::PatternString(v, _) => v.to_string(),
            Self::VariableName(v, _) => v.to_string(),
            Self::Values(v, _) => v.to_string(),
            Self::Block(v, _) => v.to_string(),
            Self::Command(v, _) => v.to_string(),
            Self::Task(v, _) => v.to_string(),
            Self::Component(v, _) => v.to_string(),
            Self::Boolean(v, _) => v.to_string(),
            Self::Integer(v, _) => v.to_string(),
            Self::VariableDeclaration(v, _) => v.to_string(),
            Self::VariableVariants(v, _) => v.to_string(),
            Self::VariableType(v, _) => v.to_string(),
            Self::SimpleString(v, _) => v.to_string(),
            Self::Ppm(v, _) => v.to_string(),
            Self::Comment(v) => v.to_string(),
            Self::Meta(v) => v.to_string(),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn as_string<A>(el: &A, md: &Metadata) -> String
        where
            A: Display,
        {
            format!(
                "{md}{}{el}{}{}",
                if md.inverting {
                    chars::EXCLAMATION.to_string()
                } else {
                    String::new()
                },
                if md.tolerance {
                    chars::QUESTION.to_string()
                } else {
                    String::new()
                },
                if let Some(call) = md.ppm.as_ref() {
                    call.to_string()
                } else {
                    String::new()
                }
            )
        }
        write!(
            f,
            "{}",
            match self {
                Self::Function(v, md) => as_string(v, md),
                Self::If(v, md) => as_string(v, md),
                Self::Breaker(v, md) => as_string(v, md),
                Self::Each(v, md) => as_string(v, md),
                Self::First(v, md) => as_string(v, md),
                Self::Join(v, md) => as_string(v, md),
                Self::VariableAssignation(v, md) => as_string(v, md),
                Self::Comparing(v, md) => as_string(v, md),
                Self::Combination(v, md) => as_string(v, md),
                Self::Condition(v, md) => as_string(v, md),
                Self::Subsequence(v, md) => as_string(v, md),
                Self::Optional(v, md) => as_string(v, md),
                Self::Gatekeeper(v, md) => as_string(v, md),
                Self::Reference(v, md) => as_string(v, md),
                Self::PatternString(v, md) => as_string(v, md),
                Self::VariableName(v, md) => as_string(v, md),
                Self::Values(v, md) => as_string(v, md),
                Self::Block(v, md) => as_string(v, md),
                Self::Command(v, md) => as_string(v, md),
                Self::Task(v, md) => as_string(v, md),
                Self::Component(v, md) => as_string(v, md),
                Self::Boolean(v, md) => as_string(v, md),
                Self::Integer(v, md) => as_string(v, md),
                Self::VariableDeclaration(v, md) => as_string(v, md),
                Self::VariableVariants(v, md) => as_string(v, md),
                Self::VariableType(v, md) => as_string(v, md),
                Self::SimpleString(v, md) => as_string(v, md),
                Self::Ppm(v, md) => as_string(v, md),
                Self::Comment(v) => v.to_string(),
                Self::Meta(v) => v.to_string(),
            }
        )
    }
}

impl Formation for Element {
    fn elements_count(&self) -> usize {
        match self {
            Self::Function(v, _) => v.elements_count(),
            Self::If(v, _) => v.elements_count(),
            Self::Breaker(v, _) => v.elements_count(),
            Self::Each(v, _) => v.elements_count(),
            Self::First(v, _) => v.elements_count(),
            Self::Join(v, _) => v.elements_count(),
            Self::VariableAssignation(v, _) => v.elements_count(),
            Self::Comparing(v, _) => v.elements_count(),
            Self::Combination(v, _) => v.elements_count(),
            Self::Condition(v, _) => v.elements_count(),
            Self::Subsequence(v, _) => v.elements_count(),
            Self::Optional(v, _) => v.elements_count(),
            Self::Gatekeeper(v, _) => v.elements_count(),
            Self::Reference(v, _) => v.elements_count(),
            Self::PatternString(v, _) => v.elements_count(),
            Self::VariableName(v, _) => v.elements_count(),
            Self::Values(v, _) => v.elements_count(),
            Self::Block(v, _) => v.elements_count(),
            Self::Command(v, _) => v.elements_count(),
            Self::Task(v, _) => v.elements_count(),
            Self::Component(v, _) => v.elements_count(),
            Self::Boolean(v, _) => v.elements_count(),
            Self::Integer(v, _) => v.elements_count(),
            Self::VariableDeclaration(v, _) => v.elements_count(),
            Self::VariableVariants(v, _) => v.elements_count(),
            Self::VariableType(v, _) => v.elements_count(),
            Self::SimpleString(v, _) => v.elements_count(),
            Self::Ppm(v, _) => v.elements_count(),
            Self::Meta(v) => v.elements_count(),
            Self::Comment(v) => v.elements_count(),
        }
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        fn format_el<A>(el: &A, md: &Metadata, cursor: &mut FormationCursor) -> String
        where
            A: Formation,
        {
            format!(
                "{}{}{}{}{}",
                md.format(cursor),
                if md.inverting {
                    chars::EXCLAMATION.to_string()
                } else {
                    String::new()
                },
                el.format(cursor),
                if md.tolerance {
                    chars::QUESTION.to_string()
                } else {
                    String::new()
                },
                if let Some(call) = md.ppm.as_ref() {
                    call.to_string()
                } else {
                    String::new()
                }
            )
        }
        match self {
            Self::Function(v, m) => format_el(v, m, cursor),
            Self::If(v, m) => format_el(v, m, cursor),
            Self::Breaker(v, m) => format_el(v, m, cursor),
            Self::Each(v, m) => format_el(v, m, cursor),
            Self::First(v, m) => format_el(v, m, cursor),
            Self::Join(v, m) => format_el(v, m, cursor),
            Self::VariableAssignation(v, m) => format_el(v, m, cursor),
            Self::Comparing(v, m) => format_el(v, m, cursor),
            Self::Combination(v, m) => format_el(v, m, cursor),
            Self::Condition(v, m) => format_el(v, m, cursor),
            Self::Subsequence(v, m) => format_el(v, m, cursor),
            Self::Optional(v, m) => format_el(v, m, cursor),
            Self::Gatekeeper(v, m) => format_el(v, m, cursor),
            Self::Reference(v, m) => format_el(v, m, cursor),
            Self::PatternString(v, m) => format_el(v, m, cursor),
            Self::VariableName(v, m) => format_el(v, m, cursor),
            Self::Values(v, m) => format_el(v, m, cursor),
            Self::Block(v, m) => format_el(v, m, cursor),
            Self::Command(v, m) => format_el(v, m, cursor),
            Self::Task(v, m) => format_el(v, m, cursor),
            Self::Component(v, m) => format_el(v, m, cursor),
            Self::Boolean(v, m) => format_el(v, m, cursor),
            Self::Integer(v, m) => format_el(v, m, cursor),
            Self::VariableDeclaration(v, m) => format_el(v, m, cursor),
            Self::VariableVariants(v, m) => format_el(v, m, cursor),
            Self::VariableType(v, m) => format_el(v, m, cursor),
            Self::SimpleString(v, m) => format_el(v, m, cursor),
            Self::Ppm(v, m) => format_el(v, m, cursor),
            Self::Meta(v) => v.format(cursor),
            Self::Comment(v) => v.format(cursor),
        }
    }
}

impl TokenGetter for Element {
    fn token(&self) -> usize {
        match self {
            Self::Function(v, _) => v.token(),
            Self::If(v, _) => v.token(),
            Self::Breaker(v, _) => v.token(),
            Self::Each(v, _) => v.token(),
            Self::First(v, _) => v.token(),
            Self::Join(v, _) => v.token(),
            Self::VariableAssignation(v, _) => v.token(),
            Self::Comparing(v, _) => v.token(),
            Self::Combination(v, _) => v.token(),
            Self::Condition(v, _) => v.token(),
            Self::Subsequence(v, _) => v.token(),
            Self::Optional(v, _) => v.token(),
            Self::Gatekeeper(v, _) => v.token(),
            Self::Reference(v, _) => v.token(),
            Self::PatternString(v, _) => v.token(),
            Self::VariableName(v, _) => v.token(),
            Self::Values(v, _) => v.token(),
            Self::Block(v, _) => v.token(),
            Self::Command(v, _) => v.token(),
            Self::Task(v, _) => v.token(),
            Self::Component(v, _) => v.token(),
            Self::Integer(v, _) => v.token(),
            Self::Boolean(v, _) => v.token(),
            Self::VariableDeclaration(v, _) => v.token,
            Self::VariableVariants(v, _) => v.token,
            Self::VariableType(v, _) => v.token,
            Self::SimpleString(v, _) => v.token(),
            Self::Ppm(v, _) => v.token(),
            Self::Meta(v) => v.token,
            Self::Comment(v) => v.token,
        }
    }
}

impl ExpectedValueType for Element {
    fn varification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move {
            match self {
                Self::Function(v, _) => v.varification(owner, components, cx).await,
                Self::If(v, _) => v.varification(owner, components, cx).await,
                Self::Breaker(v, _) => v.varification(owner, components, cx).await,
                Self::Each(v, _) => v.varification(owner, components, cx).await,
                Self::First(v, _) => v.varification(owner, components, cx).await,
                Self::Join(v, _) => v.varification(owner, components, cx).await,
                Self::VariableAssignation(v, _) => v.varification(owner, components, cx).await,
                Self::Comparing(v, _) => v.varification(owner, components, cx).await,
                Self::Combination(v, _) => v.varification(owner, components, cx).await,
                Self::Condition(v, _) => v.varification(owner, components, cx).await,
                Self::Subsequence(v, _) => v.varification(owner, components, cx).await,
                Self::Optional(v, _) => v.varification(owner, components, cx).await,
                Self::Gatekeeper(v, _) => v.varification(owner, components, cx).await,
                Self::Reference(v, _) => v.varification(owner, components, cx).await,
                Self::PatternString(v, _) => v.varification(owner, components, cx).await,
                Self::VariableName(v, _) => v.varification(owner, components, cx).await,
                Self::Values(v, _) => v.varification(owner, components, cx).await,
                Self::Block(v, _) => v.varification(owner, components, cx).await,
                Self::Command(v, _) => v.varification(owner, components, cx).await,
                Self::Task(v, _) => v.varification(owner, components, cx).await,
                Self::Component(v, _) => v.varification(self, components, cx).await,
                Self::Integer(v, _) => v.varification(owner, components, cx).await,
                Self::Boolean(v, _) => v.varification(owner, components, cx).await,
                Self::VariableDeclaration(v, _) => v.varification(owner, components, cx).await,
                Self::VariableVariants(v, _) => v.varification(owner, components, cx).await,
                Self::VariableType(v, _) => v.varification(owner, components, cx).await,
                Self::SimpleString(v, _) => v.varification(owner, components, cx).await,
                Self::Ppm(v, _) => v.varification(owner, components, cx).await,
                Self::Meta(..) => Err(operator::E::NoReturnType.by(self)),
                Self::Comment(..) => Err(operator::E::NoReturnType.by(self)),
            }
        })
    }
    fn linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            match self {
                Self::Function(v, _) => v.linking(variables, owner, components, cx).await,
                Self::If(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Breaker(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Each(v, _) => v.linking(variables, owner, components, cx).await,
                Self::First(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Join(v, _) => v.linking(variables, owner, components, cx).await,
                Self::VariableAssignation(v, _) => {
                    v.linking(variables, owner, components, cx).await
                }
                Self::Comparing(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Combination(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Condition(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Subsequence(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Optional(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Gatekeeper(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Reference(v, _) => v.linking(variables, owner, components, cx).await,
                Self::PatternString(v, _) => v.linking(variables, owner, components, cx).await,
                Self::VariableName(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Values(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Block(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Command(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Task(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Component(v, _) => v.linking(variables, self, components, cx).await,
                Self::Integer(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Boolean(v, _) => v.linking(variables, owner, components, cx).await,
                Self::VariableDeclaration(v, _) => {
                    v.linking(variables, owner, components, cx).await
                }
                Self::VariableVariants(v, _) => v.linking(variables, owner, components, cx).await,
                Self::VariableType(v, _) => v.linking(variables, owner, components, cx).await,
                Self::SimpleString(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Ppm(v, _) => v.linking(variables, owner, components, cx).await,
                Self::Meta(..) => Err(operator::E::NoReturnType.by(self)),
                Self::Comment(..) => Err(operator::E::NoReturnType.by(self)),
            }
        })
    }
    fn expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move {
            match self {
                Self::Function(v, _) => v.expected(owner, components, cx).await,
                Self::If(v, _) => v.expected(owner, components, cx).await,
                Self::Breaker(v, _) => v.expected(owner, components, cx).await,
                Self::Each(v, _) => v.expected(owner, components, cx).await,
                Self::First(v, _) => v.expected(owner, components, cx).await,
                Self::Join(v, _) => v.expected(owner, components, cx).await,
                Self::VariableAssignation(v, _) => v.expected(owner, components, cx).await,
                Self::Comparing(v, _) => v.expected(owner, components, cx).await,
                Self::Combination(v, _) => v.expected(owner, components, cx).await,
                Self::Condition(v, _) => v.expected(owner, components, cx).await,
                Self::Subsequence(v, _) => v.expected(owner, components, cx).await,
                Self::Optional(v, _) => v.expected(owner, components, cx).await,
                Self::Gatekeeper(v, _) => v.expected(owner, components, cx).await,
                Self::Reference(v, _) => v.expected(owner, components, cx).await,
                Self::PatternString(v, _) => v.expected(owner, components, cx).await,
                Self::VariableName(v, _) => v.expected(owner, components, cx).await,
                Self::Values(v, _) => v.expected(owner, components, cx).await,
                Self::Block(v, _) => v.expected(owner, components, cx).await,
                Self::Command(v, _) => v.expected(owner, components, cx).await,
                Self::Task(v, _) => v.expected(owner, components, cx).await,
                Self::Component(v, _) => v.expected(owner, components, cx).await,
                Self::Integer(v, _) => v.expected(owner, components, cx).await,
                Self::Boolean(v, _) => v.expected(owner, components, cx).await,
                Self::VariableDeclaration(v, _) => v.expected(owner, components, cx).await,
                Self::VariableVariants(v, _) => v.expected(owner, components, cx).await,
                Self::VariableType(v, _) => v.expected(owner, components, cx).await,
                Self::SimpleString(v, _) => v.expected(owner, components, cx).await,
                Self::Ppm(v, _) => v.expected(owner, components, cx).await,
                Self::Meta(..) => Err(operator::E::NoReturnType.by(self)),
                Self::Comment(..) => Err(operator::E::NoReturnType.by(self)),
            }
        })
    }
}

impl TryExecute for Element {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let journal = cx.journal.clone();
            let result = match self {
                Self::Function(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::If(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Breaker(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Each(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::First(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Join(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::VariableAssignation(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Comparing(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Combination(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Condition(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Subsequence(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Optional(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Gatekeeper(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Reference(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::PatternString(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::VariableName(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Values(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Block(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Command(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Task(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Component(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Integer(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Boolean(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Meta(..) => Ok(Value::empty()),
                Self::VariableDeclaration(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::VariableVariants(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::VariableType(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::SimpleString(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Ppm(v, _) => {
                    v.try_execute(owner, components, args, prev, cx, sc, token)
                        .await
                }
                Self::Comment(_) => Ok(Value::empty()),
            };
            if let (true, Err(err)) = (self.get_metadata().tolerance, result.as_ref()) {
                journal.as_tolerant(&err.uuid);
                return Ok(Value::empty());
            }
            let output = result?;
            Ok(
                if self.get_metadata().inverting && matches!(self, Element::Function(..)) {
                    Value::bool(
                        !output
                            .not_empty_or(operator::E::InvertingOnEmptyReturn.by(self))?
                            .as_bool()
                            .ok_or(operator::E::InvertingOnNotBool.by(self))?,
                    )
                } else {
                    output
                },
            )
        })
    }
}

impl Execute for Element {
    fn get_metadata(&self) -> Result<&Metadata, LinkedErr<operator::E>> {
        Ok(self.get_metadata())
    }
}

#[cfg(test)]
mod processing {
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::{ElTarget, Element},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/tolerance.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut elements: Vec<Element> = Vec::new();
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElTarget::Task]))?
                {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    elements.push(el);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(elements)
            },
            |elements: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for el in elements.iter() {
                    el.execute(
                        None,
                        &[],
                        &[],
                        &None,
                        cx.clone(),
                        sc.clone(),
                        CancellationToken::new(),
                    )
                    .await?;
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::{
            Block, Boolean, Breaker, Combination, Command, Comment, Comparing, Component,
            Condition, Each, ElTarget, Element, First, Function, Gatekeeper, If, Integer, Join,
            Meta, Metadata, Optional, PatternString, Reference, SimpleString, Subsequence, Task,
            Values, VariableAssignation, VariableDeclaration, VariableName, VariableType,
            VariableVariants,
        },
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Metadata {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
            prop_oneof![Just(true), Just(false),]
                .prop_map(|tolerance| Metadata {
                    comments: Vec::new(),
                    meta: Vec::new(),
                    ppm: None,
                    tolerance,
                    inverting: false,
                })
                .boxed()
        }
    }

    fn generate(targets: &[ElTarget], deep: usize) -> Vec<BoxedStrategy<Element>> {
        let mut collected = Vec::new();
        if targets.contains(&ElTarget::Combination) {
            collected.push(
                Combination::arbitrary()
                    .prop_map(|el| Element::Combination(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Breaker) {
            collected.push(
                Breaker::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Breaker(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Join) {
            collected.push(
                Join::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Join(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Subsequence) {
            collected.push(
                Subsequence::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Subsequence(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Condition) {
            collected.push(
                Condition::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Condition(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Integer) {
            collected.push(
                Integer::arbitrary()
                    .prop_map(|el| Element::Integer(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Boolean) {
            collected.push(
                Boolean::arbitrary()
                    .prop_map(|el| Element::Boolean(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Block) {
            collected.push(
                Block::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Block(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Command) {
            collected.push(
                (
                    Command::arbitrary_with(deep + 1),
                    Metadata::arbitrary_with(()),
                )
                    .prop_map(|(el, md)| Element::Command(el, md))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Comparing) {
            collected.push(
                Comparing::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Comparing(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Component) {
            collected.push(
                Component::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Component(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Each) {
            collected.push(
                Each::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Each(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::First) {
            collected.push(
                First::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::First(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Function) {
            collected.push(
                (
                    Function::arbitrary_with(deep + 1),
                    Metadata::arbitrary_with(()),
                )
                    .prop_map(|(el, md)| Element::Function(el, md))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::If) {
            collected.push(
                If::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::If(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Meta) {
            collected.push(Meta::arbitrary().prop_map(Element::Meta).boxed());
        }
        if targets.contains(&ElTarget::Optional) {
            collected.push(
                Optional::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Optional(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Gatekeeper) {
            collected.push(
                Gatekeeper::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Gatekeeper(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::PatternString) {
            collected.push(
                PatternString::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::PatternString(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Reference) {
            collected.push(
                Reference::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Reference(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Task) {
            collected.push(
                Task::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Task(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Values) {
            collected.push(
                Values::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Values(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableAssignation) {
            collected.push(
                VariableAssignation::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::VariableAssignation(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableName) {
            collected.push(
                VariableName::arbitrary()
                    .prop_map(|el| Element::VariableName(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableType) {
            collected.push(
                VariableType::arbitrary()
                    .prop_map(|el| Element::VariableType(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableDeclaration) {
            collected.push(
                VariableDeclaration::arbitrary_with(deep)
                    .prop_map(|el| Element::VariableDeclaration(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::VariableVariants) {
            collected.push(
                VariableVariants::arbitrary()
                    .prop_map(|el| Element::VariableVariants(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::SimpleString) {
            collected.push(
                SimpleString::arbitrary()
                    .prop_map(|el| Element::SimpleString(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElTarget::Comment) {
            collected.push(Comment::arbitrary().prop_map(Element::Comment).boxed());
        }
        collected
    }

    impl Arbitrary for Element {
        type Parameters = (Vec<ElTarget>, usize);
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with((targets, deep): Self::Parameters) -> Self::Strategy {
            prop::strategy::Union::new(generate(&targets, deep)).boxed()
        }
    }

    fn reading(el: Element) {
        get_rt().block_on(async {
            let origin = format!("{el};");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    while let Some(block) = src.report_err_if(Block::dissect(reader))? {
                        assert_eq!(format!("{block};"), origin);
                    }
                    Ok::<(), LinkedErr<E>>(())
                }
            );
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            max_shrink_iters: 5000,
            ..ProptestConfig::with_cases(10)
        })]
        #[test]
        fn test_run_task(
            args in any_with::<Element>((vec![ElTarget::Function], 0))
        ) {
            reading(args.clone());
        }
    }
}
