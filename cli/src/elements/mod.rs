mod accessor;
mod block;
mod call;
mod closure;
mod comment;
mod component;
mod conditions;
mod function;
mod gatekeeper;
mod meta;
mod optional;
mod primitives;
mod range;
mod reference;
mod statements;
mod string;
mod task;
mod values;
mod variable;

pub use accessor::*;
pub use block::*;
pub use call::*;
pub use closure::*;
pub use comment::*;
pub use component::*;
pub use conditions::*;
pub use function::*;
pub use gatekeeper::*;
pub use meta::*;
pub use optional::*;
pub use primitives::*;
pub use range::*;
pub use reference::*;
pub use statements::*;
pub use string::*;
pub use task::*;
pub use values::*;
pub use variable::*;

use crate::{
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult,
        ExpectedValueType, Formation, FormationCursor, LinkingResult, PrevValueExpectation,
        Processing, TryExecute, TryExpectedValueType, Value, VerificationResult,
    },
    reader::{chars, Dissect, Reader, E},
};
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ElementRef {
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

impl fmt::Display for ElementRef {
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

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub comments: Vec<Element>,
    pub meta: Vec<Element>,
    pub ppm: Option<Box<Element>>,
    pub tolerance: bool,
    pub inverting: bool,
    pub token: usize,
}

impl Metadata {
    pub fn empty() -> Self {
        Metadata {
            comments: Vec::new(),
            meta: Vec::new(),
            ppm: None,
            tolerance: false,
            inverting: false,
            token: 0,
        }
    }
    #[cfg(test)]
    pub fn comments(&self) -> Vec<&Comment> {
        self.comments
            .iter()
            .filter_map(|el| {
                if let Element::Comment(comment) = el {
                    Some(comment)
                } else {
                    None
                }
            })
            .collect::<Vec<&Comment>>()
    }
    pub fn meta(&self) -> Vec<&Meta> {
        self.meta
            .iter()
            .filter_map(|el| {
                if let Element::Meta(md) = el {
                    Some(md)
                } else {
                    None
                }
            })
            .collect::<Vec<&Meta>>()
    }
    pub fn meta_as_lines(&self) -> Vec<&str> {
        self.meta().iter().flat_map(|el| el.as_lines()).collect()
    }
    pub fn set_ppm(&mut self, el: Element) -> &mut Self {
        self.ppm = Some(Box::new(el));
        self
    }
    pub fn set_token(&mut self, token: usize) -> &mut Self {
        self.token = token;
        self
    }
    pub fn get_token(&self) -> usize {
        self.token
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

pub trait TokenGetter {
    fn token(&self) -> usize;
}

#[cfg(test)]
pub trait ElementRefGetter {
    fn get_alias(&self) -> ElementRef;
}

#[cfg(test)]
pub trait InnersGetter {
    fn get_inners(&self) -> Vec<&Element>;
}

#[derive(Debug, Clone)]
pub enum Element {
    Call(Call, Metadata),
    Accessor(Accessor, Metadata),
    Function(Function, Metadata),
    If(If, Metadata),
    IfCondition(IfCondition, Metadata),
    IfSubsequence(IfSubsequence, Metadata),
    IfThread(IfThread, Metadata),
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
    Range(Range, Metadata),
    For(For, Metadata),
    Compute(Compute, Metadata),
    Return(Return, Metadata),
    Error(Error, Metadata),
    Incrementer(Incrementer, Metadata),
    Loop(Loop, Metadata),
    While(While, Metadata),
    Closure(Closure, Metadata),
    Conclusion(Conclusion, Metadata),
    Meta(Meta),
    Comment(Comment),
}

impl Element {
    fn parse(
        reader: &mut Reader,
        targets: &[ElementRef],
        includes: bool,
    ) -> Result<Option<Element>, LinkedErr<E>> {
        fn tolerance(reader: &mut Reader, mut md: Metadata) -> Metadata {
            md.tolerance = reader.move_to().char(&[&chars::QUESTION]).is_some();
            md
        }
        fn next(
            reader: &mut Reader,
            mut el: Element,
            token: impl Fn(&mut Reader) -> usize,
        ) -> Result<Option<Element>, LinkedErr<E>> {
            let Some(ppm) = Element::include(reader, &[ElementRef::Call, ElementRef::Accessor])?
            else {
                el.get_mut_metadata().set_token(token(reader));
                return Ok(Some(el));
            };
            el.get_mut_metadata().set_ppm(ppm).set_token(token(reader));
            Ok(Some(el))
        }
        let mut comments: Vec<Element> = Vec::new();
        let mut meta: Vec<Element> = Vec::new();
        let token = reader.open_unbound_token();
        loop {
            if let Some(el) = Comment::dissect(reader)? {
                comments.push(Element::Comment(el));
                continue;
            }
            if let Some(el) = Meta::dissect(reader)? {
                meta.push(Element::Meta(el));
                continue;
            }
            break;
        }
        let md = Metadata {
            comments,
            meta,
            ppm: None,
            tolerance: false,
            inverting: if targets.contains(&ElementRef::Function) {
                reader.move_to().char(&[&chars::EXCLAMATION]).is_some()
            } else {
                false
            },
            token: 0,
        };
        if includes == targets.contains(&ElementRef::Closure) {
            if let Some(el) = Closure::dissect(reader)? {
                return next(reader, Element::Closure(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Return) {
            if let Some(el) = Return::dissect(reader)? {
                return next(reader, Element::Return(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Error) {
            if let Some(el) = Error::dissect(reader)? {
                return next(reader, Element::Error(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Compute) {
            if let Some(el) = Compute::dissect(reader)? {
                return next(reader, Element::Compute(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Loop) {
            if let Some(el) = Loop::dissect(reader)? {
                return next(reader, Element::Loop(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::While) {
            if let Some(el) = While::dissect(reader)? {
                return next(reader, Element::While(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::For) {
            if let Some(el) = For::dissect(reader)? {
                return next(reader, Element::For(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Range) {
            if let Some(el) = Range::dissect(reader)? {
                return next(reader, Element::Range(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Breaker) {
            if let Some(el) = Breaker::dissect(reader)? {
                return next(reader, Element::Breaker(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Accessor) {
            if let Some(el) = Accessor::dissect(reader)? {
                return next(reader, Element::Accessor(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Call) {
            if let Some(el) = Call::dissect(reader)? {
                return next(reader, Element::Call(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Optional) {
            if let Some(el) = Optional::dissect(reader)? {
                return next(reader, Element::Optional(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Conclusion) {
            if let Some(el) = Conclusion::dissect(reader)? {
                return next(reader, Element::Conclusion(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Combination) {
            if let Some(el) = Combination::dissect(reader)? {
                return next(reader, Element::Combination(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Subsequence) {
            if let Some(el) = Subsequence::dissect(reader)? {
                return next(reader, Element::Subsequence(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Condition) {
            if let Some(el) = Condition::dissect(reader)? {
                return next(reader, Element::Condition(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Meta) {
            if let Some(el) = Meta::dissect(reader)? {
                return next(reader, Element::Meta(el), token);
            }
        }
        if includes == targets.contains(&ElementRef::Comparing) {
            if let Some(el) = Comparing::dissect(reader)? {
                return next(reader, Element::Comparing(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::If) {
            if let Some(el) = If::dissect(reader)? {
                return next(reader, Element::If(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::IfThread) {
            if let Some(el) = IfThread::dissect(reader)? {
                return next(reader, Element::IfThread(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::IfSubsequence) {
            if let Some(el) = IfSubsequence::dissect(reader)? {
                return next(reader, Element::IfSubsequence(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::IfCondition) {
            if let Some(el) = IfCondition::dissect(reader)? {
                return next(reader, Element::IfCondition(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Gatekeeper) {
            if let Some(el) = Gatekeeper::dissect(reader)? {
                return next(reader, Element::Gatekeeper(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Command) {
            if let Some(el) = Command::dissect(reader)? {
                let to = tolerance(reader, md);
                return next(reader, Element::Command(el, to), token);
            }
        }
        if includes == targets.contains(&ElementRef::Integer) {
            if let Some(el) = Integer::dissect(reader)? {
                return next(reader, Element::Integer(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Boolean) {
            if let Some(el) = Boolean::dissect(reader)? {
                return next(reader, Element::Boolean(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Incrementer) {
            if let Some(el) = Incrementer::dissect(reader)? {
                return next(reader, Element::Incrementer(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::VariableAssignation) {
            if let Some(el) = VariableAssignation::dissect(reader)? {
                return next(reader, Element::VariableAssignation(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Each) {
            if let Some(el) = Each::dissect(reader)? {
                return next(reader, Element::Each(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::First) {
            if let Some(el) = First::dissect(reader)? {
                return next(reader, Element::First(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Join) {
            if let Some(el) = Join::dissect(reader)? {
                return next(reader, Element::Join(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Function) {
            if let Some(el) = Function::dissect(reader)? {
                let to = tolerance(reader, md);
                return next(reader, Element::Function(el, to), token);
            }
        }
        if includes == targets.contains(&ElementRef::Reference) {
            if let Some(el) = Reference::dissect(reader)? {
                return next(reader, Element::Reference(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::PatternString) {
            if let Some(el) = PatternString::dissect(reader)? {
                return next(reader, Element::PatternString(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Block) {
            if let Some(el) = Block::dissect(reader)? {
                return next(reader, Element::Block(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Values) {
            if let Some(el) = Values::dissect(reader)? {
                return next(reader, Element::Values(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::VariableName) {
            if let Some(el) = VariableName::dissect(reader)? {
                return next(reader, Element::VariableName(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Component) {
            if let Some(el) = Component::dissect(reader)? {
                return next(reader, Element::Component(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::Task) {
            if let Some(el) = Task::dissect(reader)? {
                return next(reader, Element::Task(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::VariableDeclaration) {
            if let Some(el) = VariableDeclaration::dissect(reader)? {
                return next(reader, Element::VariableDeclaration(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::VariableType) {
            if let Some(el) = VariableType::dissect(reader)? {
                return next(reader, Element::VariableType(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::VariableVariants) {
            if let Some(el) = VariableVariants::dissect(reader)? {
                return next(reader, Element::VariableVariants(el, md), token);
            }
        }
        if includes == targets.contains(&ElementRef::SimpleString) {
            if let Some(el) = SimpleString::dissect(reader)? {
                return next(reader, Element::SimpleString(el, md), token);
            }
        }
        Ok(None)
    }

    pub fn exclude(
        reader: &mut Reader,
        targets: &[ElementRef],
    ) -> Result<Option<Element>, LinkedErr<E>> {
        Self::parse(reader, targets, false)
    }

    pub fn include(
        reader: &mut Reader,
        targets: &[ElementRef],
    ) -> Result<Option<Element>, LinkedErr<E>> {
        Self::parse(reader, targets, true)
    }

    pub fn get_metadata(&self) -> &Metadata {
        match self {
            Self::Call(_, md) => md,
            Self::Accessor(_, md) => md,
            Self::Function(_, md) => md,
            Self::If(_, md) => md,
            Self::IfCondition(_, md) => md,
            Self::IfSubsequence(_, md) => md,
            Self::IfThread(_, md) => md,
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
            Self::Range(_, md) => md,
            Self::For(_, md) => md,
            Self::Compute(_, md) => md,
            Self::Return(_, md) => md,
            Self::Error(_, md) => md,
            Self::Incrementer(_, md) => md,
            Self::Loop(_, md) => md,
            Self::While(_, md) => md,
            Self::Closure(_, md) => md,
            Self::Conclusion(_, md) => md,
            Self::Comment(_) | Self::Meta(_) => {
                panic!("Comment doesn't have metadata");
            }
        }
    }

    pub fn get_mut_metadata(&mut self) -> &mut Metadata {
        match self {
            Self::Call(_, md) => md,
            Self::Accessor(_, md) => md,
            Self::Function(_, md) => md,
            Self::If(_, md) => md,
            Self::IfCondition(_, md) => md,
            Self::IfSubsequence(_, md) => md,
            Self::IfThread(_, md) => md,
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
            Self::Range(_, md) => md,
            Self::For(_, md) => md,
            Self::Compute(_, md) => md,
            Self::Return(_, md) => md,
            Self::Error(_, md) => md,
            Self::Incrementer(_, md) => md,
            Self::Loop(_, md) => md,
            Self::While(_, md) => md,
            Self::Closure(_, md) => md,
            Self::Conclusion(_, md) => md,
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

    pub fn drop_ppm(&mut self, reader: &mut Reader) -> Result<(), E> {
        let meta = self.get_mut_metadata();
        if let Some(ppm) = &meta.ppm {
            reader.drop_to(ppm.token())?;
        }
        Ok(())
    }

    pub fn inner_token(&self) -> usize {
        match self {
            Self::Call(v, _) => v.token(),
            Self::Accessor(v, _) => v.token(),
            Self::Function(v, _) => v.token(),
            Self::If(v, _) => v.token(),
            Self::IfCondition(v, _) => v.token(),
            Self::IfSubsequence(v, _) => v.token(),
            Self::IfThread(v, _) => v.token(),
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
            Self::Range(v, _) => v.token(),
            Self::For(v, _) => v.token(),
            Self::Compute(v, _) => v.token(),
            Self::Return(v, _) => v.token(),
            Self::Error(v, _) => v.token(),
            Self::Incrementer(v, _) => v.token(),
            Self::Loop(v, _) => v.token(),
            Self::While(v, _) => v.token(),
            Self::Closure(v, _) => v.token(),
            Self::Conclusion(v, _) => v.token(),
            Self::Meta(v) => v.token,
            Self::Comment(v) => v.token,
        }
    }

    #[cfg(test)]
    pub fn inner_to_string(&self) -> String {
        match self {
            Self::Call(v, _) => v.to_string(),
            Self::Accessor(v, _) => v.to_string(),
            Self::Function(v, _) => v.to_string(),
            Self::If(v, _) => v.to_string(),
            Self::IfCondition(v, _) => v.to_string(),
            Self::IfSubsequence(v, _) => v.to_string(),
            Self::IfThread(v, _) => v.to_string(),
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
            Self::Range(v, _) => v.to_string(),
            Self::For(v, _) => v.to_string(),
            Self::Compute(v, _) => v.to_string(),
            Self::Return(v, _) => v.to_string(),
            Self::Error(v, _) => v.to_string(),
            Self::Incrementer(v, _) => v.to_string(),
            Self::Loop(v, _) => v.to_string(),
            Self::While(v, _) => v.to_string(),
            Self::Closure(v, _) => v.to_string(),
            Self::Conclusion(v, _) => v.to_string(),
            Self::Comment(v) => v.to_string(),
            Self::Meta(v) => v.to_string(),
        }
    }
}

#[cfg(test)]
impl InnersGetter for Element {
    fn get_inners(&self) -> Vec<&Element> {
        match self {
            Self::Call(v, _) => v.get_inners(),
            Self::Accessor(v, _) => v.get_inners(),
            Self::Function(v, _) => v.get_inners(),
            Self::If(v, _) => v.get_inners(),
            Self::IfCondition(v, _) => v.get_inners(),
            Self::IfSubsequence(v, _) => v.get_inners(),
            Self::IfThread(v, _) => v.get_inners(),
            Self::Breaker(v, _) => v.get_inners(),
            Self::Each(v, _) => v.get_inners(),
            Self::First(v, _) => v.get_inners(),
            Self::Join(v, _) => v.get_inners(),
            Self::VariableAssignation(v, _) => v.get_inners(),
            Self::Comparing(v, _) => v.get_inners(),
            Self::Combination(v, _) => v.get_inners(),
            Self::Condition(v, _) => v.get_inners(),
            Self::Subsequence(v, _) => v.get_inners(),
            Self::Optional(v, _) => v.get_inners(),
            Self::Gatekeeper(v, _) => v.get_inners(),
            Self::Reference(v, _) => v.get_inners(),
            Self::PatternString(v, _) => v.get_inners(),
            Self::VariableName(v, _) => v.get_inners(),
            Self::Values(v, _) => v.get_inners(),
            Self::Block(v, _) => v.get_inners(),
            Self::Command(v, _) => v.get_inners(),
            Self::Task(v, _) => v.get_inners(),
            Self::Component(v, _) => v.get_inners(),
            Self::Boolean(v, _) => v.get_inners(),
            Self::Integer(v, _) => v.get_inners(),
            Self::VariableDeclaration(v, _) => v.get_inners(),
            Self::VariableVariants(v, _) => v.get_inners(),
            Self::VariableType(v, _) => v.get_inners(),
            Self::SimpleString(v, _) => v.get_inners(),
            Self::Range(v, _) => v.get_inners(),
            Self::For(v, _) => v.get_inners(),
            Self::Compute(v, _) => v.get_inners(),
            Self::Return(v, _) => v.get_inners(),
            Self::Error(v, _) => v.get_inners(),
            Self::Incrementer(v, _) => v.get_inners(),
            Self::Loop(v, _) => v.get_inners(),
            Self::While(v, _) => v.get_inners(),
            Self::Closure(v, _) => v.get_inners(),
            Self::Conclusion(v, _) => v.get_inners(),
            Self::Meta(v) => v.get_inners(),
            Self::Comment(v) => v.get_inners(),
        }
    }
}
#[cfg(test)]
impl ElementRefGetter for Element {
    #[cfg(test)]
    fn get_alias(&self) -> ElementRef {
        match self {
            Self::Call(..) => ElementRef::Call,
            Self::Accessor(..) => ElementRef::Accessor,
            Self::Function(..) => ElementRef::Function,
            Self::If(..) => ElementRef::If,
            Self::IfCondition(..) => ElementRef::IfCondition,
            Self::IfSubsequence(..) => ElementRef::IfSubsequence,
            Self::IfThread(..) => ElementRef::IfThread,
            Self::Breaker(..) => ElementRef::Breaker,
            Self::Each(..) => ElementRef::Each,
            Self::First(..) => ElementRef::First,
            Self::Join(..) => ElementRef::Join,
            Self::VariableAssignation(..) => ElementRef::VariableAssignation,
            Self::Comparing(..) => ElementRef::Comparing,
            Self::Combination(..) => ElementRef::Combination,
            Self::Condition(..) => ElementRef::Condition,
            Self::Subsequence(..) => ElementRef::Subsequence,
            Self::Optional(..) => ElementRef::Optional,
            Self::Gatekeeper(..) => ElementRef::Gatekeeper,
            Self::Reference(..) => ElementRef::Reference,
            Self::PatternString(..) => ElementRef::PatternString,
            Self::VariableName(..) => ElementRef::VariableName,
            Self::Values(..) => ElementRef::Values,
            Self::Meta(..) => ElementRef::Meta,
            Self::Block(..) => ElementRef::Block,
            Self::Command(..) => ElementRef::Command,
            Self::Task(..) => ElementRef::Task,
            Self::Component(..) => ElementRef::Component,
            Self::Boolean(..) => ElementRef::Boolean,
            Self::Integer(..) => ElementRef::Integer,
            Self::VariableDeclaration(..) => ElementRef::VariableDeclaration,
            Self::VariableVariants(..) => ElementRef::VariableVariants,
            Self::VariableType(..) => ElementRef::VariableType,
            Self::SimpleString(..) => ElementRef::SimpleString,
            Self::Range(..) => ElementRef::Range,
            Self::For(..) => ElementRef::For,
            Self::Compute(..) => ElementRef::Compute,
            Self::Return(..) => ElementRef::Return,
            Self::Error(..) => ElementRef::Error,
            Self::Incrementer(..) => ElementRef::Incrementer,
            Self::Loop(..) => ElementRef::Loop,
            Self::While(..) => ElementRef::While,
            Self::Closure(..) => ElementRef::Closure,
            Self::Conclusion(..) => ElementRef::Conclusion,
            Self::Comment(..) => ElementRef::Comment,
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
                Self::Call(v, md) => as_string(v, md),
                Self::Accessor(v, md) => as_string(v, md),
                Self::Function(v, md) => as_string(v, md),
                Self::If(v, md) => as_string(v, md),
                Self::IfCondition(v, md) => as_string(v, md),
                Self::IfSubsequence(v, md) => as_string(v, md),
                Self::IfThread(v, md) => as_string(v, md),
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
                Self::Range(v, md) => as_string(v, md),
                Self::For(v, md) => as_string(v, md),
                Self::Compute(v, md) => as_string(v, md),
                Self::Return(v, md) => as_string(v, md),
                Self::Error(v, md) => as_string(v, md),
                Self::Incrementer(v, md) => as_string(v, md),
                Self::Loop(v, md) => as_string(v, md),
                Self::While(v, md) => as_string(v, md),
                Self::Closure(v, md) => as_string(v, md),
                Self::Conclusion(v, md) => as_string(v, md),
                Self::Comment(v) => v.to_string(),
                Self::Meta(v) => v.to_string(),
            }
        )
    }
}

impl Formation for Element {
    fn elements_count(&self) -> usize {
        match self {
            Self::Call(v, _) => v.elements_count(),
            Self::Accessor(v, _) => v.elements_count(),
            Self::Function(v, _) => v.elements_count(),
            Self::If(v, _) => v.elements_count(),
            Self::IfCondition(v, _) => v.elements_count(),
            Self::IfSubsequence(v, _) => v.elements_count(),
            Self::IfThread(v, _) => v.elements_count(),
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
            Self::Range(v, _) => v.elements_count(),
            Self::For(v, _) => v.elements_count(),
            Self::Compute(v, _) => v.elements_count(),
            Self::Return(v, _) => v.elements_count(),
            Self::Error(v, _) => v.elements_count(),
            Self::Incrementer(v, _) => v.elements_count(),
            Self::Loop(v, _) => v.elements_count(),
            Self::While(v, _) => v.elements_count(),
            Self::Closure(v, _) => v.elements_count(),
            Self::Conclusion(v, _) => v.elements_count(),
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
            Self::Call(v, m) => format_el(v, m, cursor),
            Self::Accessor(v, m) => format_el(v, m, cursor),
            Self::Function(v, m) => format_el(v, m, cursor),
            Self::If(v, m) => format_el(v, m, cursor),
            Self::IfCondition(v, m) => format_el(v, m, cursor),
            Self::IfSubsequence(v, m) => format_el(v, m, cursor),
            Self::IfThread(v, m) => format_el(v, m, cursor),
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
            Self::Range(v, m) => format_el(v, m, cursor),
            Self::For(v, m) => format_el(v, m, cursor),
            Self::Compute(v, m) => format_el(v, m, cursor),
            Self::Return(v, m) => format_el(v, m, cursor),
            Self::Error(v, m) => format_el(v, m, cursor),
            Self::Incrementer(v, m) => format_el(v, m, cursor),
            Self::Loop(v, m) => format_el(v, m, cursor),
            Self::While(v, m) => format_el(v, m, cursor),
            Self::Closure(v, m) => format_el(v, m, cursor),
            Self::Conclusion(v, m) => format_el(v, m, cursor),
            Self::Meta(v) => v.format(cursor),
            Self::Comment(v) => v.format(cursor),
        }
    }
}

impl TokenGetter for Element {
    fn token(&self) -> usize {
        match self {
            Self::Call(_, md) => md.get_token(),
            Self::Accessor(_, md) => md.get_token(),
            Self::Function(_, md) => md.get_token(),
            Self::If(_, md) => md.get_token(),
            Self::IfCondition(_, md) => md.get_token(),
            Self::IfSubsequence(_, md) => md.get_token(),
            Self::IfThread(_, md) => md.get_token(),
            Self::Breaker(_, md) => md.get_token(),
            Self::Each(_, md) => md.get_token(),
            Self::First(_, md) => md.get_token(),
            Self::Join(_, md) => md.get_token(),
            Self::VariableAssignation(_, md) => md.get_token(),
            Self::Comparing(_, md) => md.get_token(),
            Self::Combination(_, md) => md.get_token(),
            Self::Condition(_, md) => md.get_token(),
            Self::Subsequence(_, md) => md.get_token(),
            Self::Optional(_, md) => md.get_token(),
            Self::Gatekeeper(_, md) => md.get_token(),
            Self::Reference(_, md) => md.get_token(),
            Self::PatternString(_, md) => md.get_token(),
            Self::VariableName(_, md) => md.get_token(),
            Self::Values(_, md) => md.get_token(),
            Self::Block(_, md) => md.get_token(),
            Self::Command(_, md) => md.get_token(),
            Self::Task(_, md) => md.get_token(),
            Self::Component(_, md) => md.get_token(),
            Self::Integer(_, md) => md.get_token(),
            Self::Boolean(_, md) => md.get_token(),
            Self::VariableDeclaration(_, md) => md.get_token(),
            Self::VariableVariants(_, md) => md.get_token(),
            Self::VariableType(_, md) => md.get_token(),
            Self::SimpleString(_, md) => md.get_token(),
            Self::Range(_, md) => md.get_token(),
            Self::For(_, md) => md.get_token(),
            Self::Compute(_, md) => md.get_token(),
            Self::Return(_, md) => md.get_token(),
            Self::Error(_, md) => md.get_token(),
            Self::Incrementer(_, md) => md.get_token(),
            Self::Loop(_, md) => md.get_token(),
            Self::While(_, md) => md.get_token(),
            Self::Closure(_, md) => md.get_token(),
            Self::Conclusion(_, md) => md.get_token(),
            Self::Meta(v) => v.token,
            Self::Comment(v) => v.token,
        }
    }
}

impl TryExpectedValueType for Element {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            match self {
                Self::Call(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Accessor(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Function(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::If(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::IfCondition(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::IfSubsequence(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::IfThread(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Breaker(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Each(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::First(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Join(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::VariableAssignation(v, _) => {
                    v.try_verification(owner, components, prev, cx).await
                }
                Self::Comparing(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Combination(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Condition(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Subsequence(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Optional(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Gatekeeper(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Reference(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::PatternString(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::VariableName(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Values(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Block(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Command(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Task(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Component(v, _) => v.try_verification(self, components, prev, cx).await,
                Self::Integer(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Boolean(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::VariableDeclaration(v, _) => {
                    v.try_verification(owner, components, prev, cx).await
                }
                Self::VariableVariants(v, _) => {
                    v.try_verification(owner, components, prev, cx).await
                }
                Self::VariableType(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::SimpleString(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Range(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::For(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Compute(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Return(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Error(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Incrementer(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Loop(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::While(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Closure(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Conclusion(v, _) => v.try_verification(owner, components, prev, cx).await,
                Self::Meta(..) => Err(operator::E::NoReturnType.by(self)),
                Self::Comment(..) => Err(operator::E::NoReturnType.by(self)),
            }
        })
    }
    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move {
            match self {
                Self::Call(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Accessor(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Function(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::If(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::IfCondition(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::IfSubsequence(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::IfThread(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Breaker(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Each(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::First(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Join(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::VariableAssignation(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Comparing(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Combination(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Condition(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Subsequence(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Optional(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Gatekeeper(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Reference(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::PatternString(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::VariableName(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Values(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Block(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Command(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Task(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Component(v, _) => v.try_linking(self, components, prev, cx).await,
                Self::Integer(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Boolean(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::VariableDeclaration(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::VariableVariants(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::VariableType(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::SimpleString(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Range(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::For(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Compute(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Return(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Error(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Incrementer(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Loop(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::While(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Closure(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Conclusion(v, _) => v.try_linking(owner, components, prev, cx).await,
                Self::Meta(..) => Err(operator::E::NoReturnType.by(self)),
                Self::Comment(..) => Err(operator::E::NoReturnType.by(self)),
            }
        })
    }
    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            match self {
                Self::Call(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Accessor(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Function(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::If(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::IfCondition(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::IfSubsequence(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::IfThread(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Breaker(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Each(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::First(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Join(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::VariableAssignation(v, _) => {
                    v.try_expected(owner, components, prev, cx).await
                }
                Self::Comparing(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Combination(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Condition(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Subsequence(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Optional(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Gatekeeper(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Reference(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::PatternString(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::VariableName(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Values(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Block(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Command(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Task(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Component(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Integer(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Boolean(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::VariableDeclaration(v, _) => {
                    v.try_expected(owner, components, prev, cx).await
                }
                Self::VariableVariants(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::VariableType(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::SimpleString(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Range(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::For(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Compute(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Return(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Error(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Incrementer(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Loop(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::While(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Closure(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Conclusion(v, _) => v.try_expected(owner, components, prev, cx).await,
                Self::Meta(..) => Err(operator::E::NoReturnType.by(self)),
                Self::Comment(..) => Err(operator::E::NoReturnType.by(self)),
            }
        })
    }
}

impl ExpectedValueType for Element {}

impl Processing for Element {
    fn processing<'a>(
        &'a self,
        results: &'a Value,
        cx: ExecuteContext<'a>,
    ) -> operator::ProcessingPinnedResult<'a> {
        Box::pin(async move {
            match self {
                Self::Conclusion(v, _) => v.processing(results, cx).await,
                Self::Closure(v, _) => v.processing(results, cx).await,
                Self::Loop(v, _) => v.processing(results, cx).await,
                Self::While(v, _) => v.processing(results, cx).await,
                Self::Incrementer(v, _) => v.processing(results, cx).await,
                Self::Return(v, _) => v.processing(results, cx).await,
                Self::Error(v, _) => v.processing(results, cx).await,
                Self::Compute(v, _) => v.processing(results, cx).await,
                Self::For(v, _) => v.processing(results, cx).await,
                Self::Range(v, _) => v.processing(results, cx).await,
                Self::Call(v, _) => v.processing(results, cx).await,
                Self::Accessor(v, _) => v.processing(results, cx).await,
                Self::Function(v, _) => v.processing(results, cx).await,
                Self::If(v, _) => v.processing(results, cx).await,
                Self::IfCondition(v, _) => v.processing(results, cx).await,
                Self::IfSubsequence(v, _) => v.processing(results, cx).await,
                Self::IfThread(v, _) => v.processing(results, cx).await,
                Self::Breaker(v, _) => v.processing(results, cx).await,
                Self::Each(v, _) => v.processing(results, cx).await,
                Self::First(v, _) => v.processing(results, cx).await,
                Self::Join(v, _) => v.processing(results, cx).await,
                Self::VariableAssignation(v, _) => v.processing(results, cx).await,
                Self::Comparing(v, _) => v.processing(results, cx).await,
                Self::Combination(v, _) => v.processing(results, cx).await,
                Self::Condition(v, _) => v.processing(results, cx).await,
                Self::Subsequence(v, _) => v.processing(results, cx).await,
                Self::Optional(v, _) => v.processing(results, cx).await,
                Self::Gatekeeper(v, _) => v.processing(results, cx).await,
                Self::Reference(v, _) => v.processing(results, cx).await,
                Self::PatternString(v, _) => v.processing(results, cx).await,
                Self::VariableName(v, _) => v.processing(results, cx).await,
                Self::Values(v, _) => v.processing(results, cx).await,
                Self::Block(v, _) => v.processing(results, cx).await,
                Self::Command(v, _) => v.processing(results, cx).await,
                Self::Task(v, _) => v.processing(results, cx).await,
                Self::Component(v, _) => v.processing(results, cx).await,
                Self::Integer(v, _) => v.processing(results, cx).await,
                Self::Boolean(v, _) => v.processing(results, cx).await,
                Self::VariableDeclaration(v, _) => v.processing(results, cx).await,
                Self::VariableVariants(v, _) => v.processing(results, cx).await,
                Self::VariableType(v, _) => v.processing(results, cx).await,
                Self::SimpleString(v, _) => v.processing(results, cx).await,
                Self::Meta(..) => Ok(()),
                Self::Comment(_) => Ok(()),
            }
        })
    }
}

impl TryExecute for Element {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let journal = cx.journal().clone();
            let result = match self {
                Self::Conclusion(v, _) => v.try_execute(cx.clone()).await,
                Self::Closure(v, _) => v.try_execute(cx.clone()).await,
                Self::Loop(v, _) => v.try_execute(cx.clone()).await,
                Self::While(v, _) => v.try_execute(cx.clone()).await,
                Self::Incrementer(v, _) => v.try_execute(cx.clone()).await,
                Self::Return(v, _) => v.try_execute(cx.clone()).await,
                Self::Error(v, _) => v.try_execute(cx.clone()).await,
                Self::Compute(v, _) => v.try_execute(cx.clone()).await,
                Self::For(v, _) => v.try_execute(cx.clone()).await,
                Self::Range(v, _) => v.try_execute(cx.clone()).await,
                Self::Call(v, _) => v.try_execute(cx.clone()).await,
                Self::Accessor(v, _) => v.try_execute(cx.clone()).await,
                Self::Function(v, _) => v.try_execute(cx.clone()).await,
                Self::If(v, _) => v.try_execute(cx.clone()).await,
                Self::IfCondition(v, _) => v.try_execute(cx.clone()).await,
                Self::IfSubsequence(v, _) => v.try_execute(cx.clone()).await,
                Self::IfThread(v, _) => v.try_execute(cx.clone()).await,
                Self::Breaker(v, _) => v.try_execute(cx.clone()).await,
                Self::Each(v, _) => v.try_execute(cx.clone()).await,
                Self::First(v, _) => v.try_execute(cx.clone()).await,
                Self::Join(v, _) => v.try_execute(cx.clone()).await,
                Self::VariableAssignation(v, _) => v.try_execute(cx.clone()).await,
                Self::Comparing(v, _) => v.try_execute(cx.clone()).await,
                Self::Combination(v, _) => v.try_execute(cx.clone()).await,
                Self::Condition(v, _) => v.try_execute(cx.clone()).await,
                Self::Subsequence(v, _) => v.try_execute(cx.clone()).await,
                Self::Optional(v, _) => v.try_execute(cx.clone()).await,
                Self::Gatekeeper(v, _) => v.try_execute(cx.clone()).await,
                Self::Reference(v, _) => v.try_execute(cx.clone()).await,
                Self::PatternString(v, _) => v.try_execute(cx.clone()).await,
                Self::VariableName(v, _) => v.try_execute(cx.clone()).await,
                Self::Values(v, _) => v.try_execute(cx.clone()).await,
                Self::Block(v, _) => v.try_execute(cx.clone()).await,
                Self::Command(v, _) => v.try_execute(cx.clone()).await,
                Self::Task(v, _) => v.try_execute(cx.clone()).await,
                Self::Component(v, _) => v.try_execute(cx.clone()).await,
                Self::Integer(v, _) => v.try_execute(cx.clone()).await,
                Self::Boolean(v, _) => v.try_execute(cx.clone()).await,
                Self::Meta(..) => Ok(Value::empty()),
                Self::VariableDeclaration(v, _) => v.try_execute(cx.clone()).await,
                Self::VariableVariants(v, _) => v.try_execute(cx.clone()).await,
                Self::VariableType(v, _) => v.try_execute(cx.clone()).await,
                Self::SimpleString(v, _) => v.try_execute(cx.clone()).await,
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

    use crate::{
        elements::{Element, ElementRef},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    #[tokio::test]
    async fn processing() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/tolerance.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut elements: Vec<Element> = Vec::new();
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    elements.push(el);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(elements)
            },
            |elements: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for el in elements.iter() {
                    el.execute(ExecuteContext::unbound(cx.clone(), sc.clone()))
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
            Accessor, Block, Boolean, Breaker, Call, Closure, Combination, Command, Comment,
            Comparing, Component, Compute, Conclusion, Condition, Each, Element, ElementRef, Error,
            First, For, Function, Gatekeeper, If, IfCondition, IfSubsequence, IfThread,
            Incrementer, Integer, Join, Loop, Meta, Metadata, Optional, PatternString, Range,
            Reference, Return, SimpleString, Subsequence, Task, Values, VariableAssignation,
            VariableDeclaration, VariableName, VariableType, VariableVariants, While,
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
                    token: 0,
                })
                .boxed()
        }
    }

    fn generate(targets: &[ElementRef], deep: usize) -> Vec<BoxedStrategy<Element>> {
        let mut collected = Vec::new();
        if targets.contains(&ElementRef::Range) {
            collected.push(
                Range::arbitrary()
                    .prop_map(|el| Element::Range(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Call) {
            collected.push(
                Call::arbitrary()
                    .prop_map(|el| Element::Call(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Accessor) {
            collected.push(
                Accessor::arbitrary()
                    .prop_map(|el| Element::Accessor(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Combination) {
            collected.push(
                Combination::arbitrary()
                    .prop_map(|el| Element::Combination(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Conclusion) {
            collected.push(
                Conclusion::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Conclusion(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Closure) {
            collected.push(
                Closure::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Closure(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Loop) {
            collected.push(
                Loop::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Loop(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::While) {
            collected.push(
                While::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::While(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Incrementer) {
            collected.push(
                Incrementer::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Incrementer(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Error) {
            collected.push(
                Error::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Error(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Return) {
            collected.push(
                Return::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Return(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Compute) {
            collected.push(
                Compute::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Compute(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Breaker) {
            collected.push(
                Breaker::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Breaker(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Join) {
            collected.push(
                Join::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Join(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Subsequence) {
            collected.push(
                Subsequence::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Subsequence(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Condition) {
            collected.push(
                Condition::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Condition(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Integer) {
            collected.push(
                Integer::arbitrary()
                    .prop_map(|el| Element::Integer(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Boolean) {
            collected.push(
                Boolean::arbitrary()
                    .prop_map(|el| Element::Boolean(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Block) {
            collected.push(
                Block::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Block(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Command) {
            collected.push(
                (
                    Command::arbitrary_with(deep + 1),
                    Metadata::arbitrary_with(()),
                )
                    .prop_map(|(el, md)| Element::Command(el, md))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Comparing) {
            collected.push(
                Comparing::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Comparing(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Component) {
            collected.push(
                Component::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Component(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Each) {
            collected.push(
                Each::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Each(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::First) {
            collected.push(
                First::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::First(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::For) {
            collected.push(
                For::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::For(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Function) {
            collected.push(
                (
                    Function::arbitrary_with(deep + 1),
                    Metadata::arbitrary_with(()),
                )
                    .prop_map(|(el, md)| Element::Function(el, md))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::If) {
            collected.push(
                If::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::If(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::IfThread) {
            collected.push(
                IfThread::arbitrary_with((0, deep + 1))
                    .prop_map(|el| Element::IfThread(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::IfCondition) {
            collected.push(
                IfCondition::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::IfCondition(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::IfSubsequence) {
            collected.push(
                IfSubsequence::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::IfSubsequence(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Meta) {
            collected.push(Meta::arbitrary().prop_map(Element::Meta).boxed());
        }
        if targets.contains(&ElementRef::Optional) {
            collected.push(
                Optional::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Optional(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Gatekeeper) {
            collected.push(
                Gatekeeper::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Gatekeeper(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::PatternString) {
            collected.push(
                PatternString::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::PatternString(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Reference) {
            collected.push(
                Reference::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Reference(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Task) {
            collected.push(
                Task::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Task(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Values) {
            collected.push(
                Values::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::Values(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::VariableAssignation) {
            collected.push(
                VariableAssignation::arbitrary_with(deep + 1)
                    .prop_map(|el| Element::VariableAssignation(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::VariableName) {
            collected.push(
                VariableName::arbitrary()
                    .prop_map(|el| Element::VariableName(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::VariableType) {
            collected.push(
                VariableType::arbitrary()
                    .prop_map(|el| Element::VariableType(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::VariableDeclaration) {
            collected.push(
                VariableDeclaration::arbitrary_with(deep)
                    .prop_map(|el| Element::VariableDeclaration(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::VariableVariants) {
            collected.push(
                VariableVariants::arbitrary()
                    .prop_map(|el| Element::VariableVariants(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::SimpleString) {
            collected.push(
                SimpleString::arbitrary()
                    .prop_map(|el| Element::SimpleString(el, Metadata::default()))
                    .boxed(),
            );
        }
        if targets.contains(&ElementRef::Comment) {
            collected.push(Comment::arbitrary().prop_map(Element::Comment).boxed());
        }
        collected
    }

    impl Arbitrary for Element {
        type Parameters = (Vec<ElementRef>, usize);
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
            args in any_with::<Element>((vec![ElementRef::Function], 0))
        ) {
            reading(args.clone());
        }
    }
}

#[cfg(test)]
mod ppm {
    use crate::{elements::ElementRef, test_reading_el_by_el};

    test_reading_el_by_el!(
        reading,
        &include_str!("../tests/reading/ppm.sibs"),
        &[ElementRef::Function, ElementRef::VariableName],
        94
    );

    #[cfg(test)]
    mod proptest {

        use crate::{
            elements::{Accessor, Call, Task},
            error::LinkedErr,
            inf::{operator::E, tests::*, Configuration},
            read_string,
            reader::{Dissect, Reader, Sources},
        };
        use proptest::prelude::*;

        fn reading_call(call: Call) {
            get_rt().block_on(async {
                let origin = format!("@test {{\nsome_initial_func(){call};\n}};");
                read_string!(
                    &Configuration::logs(false),
                    &origin,
                    |reader: &mut Reader, src: &mut Sources| {
                        let task = src
                            .report_err_if(Task::dissect(reader))?
                            .expect("Task read");
                        assert_eq!(format!("{task};"), origin);
                        Ok::<(), LinkedErr<E>>(())
                    }
                );
            })
        }

        fn reading_accessor(acs: Accessor) {
            get_rt().block_on(async {
                let origin = format!("@test {{\nsome_initial_func(){acs};\n}};");
                read_string!(
                    &Configuration::logs(false),
                    &origin,
                    |reader: &mut Reader, src: &mut Sources| {
                        let task = src
                            .report_err_if(Task::dissect(reader))?
                            .expect("Task read");
                        assert_eq!(format!("{task};"), origin);
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
            fn test_run_calls(
                args in any_with::<Call>(0)
            ) {
                reading_call(args.clone());
            }
            #[test]
            fn test_run_accessors(
                args in any_with::<Accessor>(0)
            ) {
                reading_accessor(args.clone());
            }
        }
    }
}
