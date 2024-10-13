mod accessor;
mod block;
mod call;
mod closure;
mod comment;
mod component;
mod conditions;
mod executing;
mod formation;
mod function;
mod gatekeeper;
mod ids;
mod interfaces;
mod md;
mod meta;
mod optional;
mod primitives;
#[cfg(test)]
mod proptests;
mod range;
mod reference;
mod requirements;
mod statements;
mod string;
mod task;
#[cfg(test)]
mod tests;
mod values;
mod variable;
mod verification;

pub use accessor::*;
pub use block::*;
pub use call::*;
pub use closure::*;
pub use comment::*;
pub use component::*;
pub use conditions::*;
pub use function::*;
pub use gatekeeper::*;
pub use ids::*;
pub use md::*;
pub use meta::*;
pub use optional::*;
pub use primitives::*;
pub use range::*;
pub use reference::*;
pub use requirements::*;
pub use statements::*;
pub use string::*;
pub use task::*;
pub use values::*;
pub use variable::*;

use crate::{
    error::LinkedErr,
    inf::operator,
    reader::{chars, Dissect, Reader, E},
};

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
        targets: &[ElementId],
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
            let Some(ppm) = Element::include(reader, &[ElementId::Call, ElementId::Accessor])?
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
            inverting: if targets.contains(&ElementId::Function) {
                reader.move_to().char(&[&chars::EXCLAMATION]).is_some()
            } else {
                false
            },
            token: 0,
        };
        if includes == targets.contains(&ElementId::Closure) {
            if let Some(el) = Closure::dissect(reader)? {
                return next(reader, Element::Closure(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Return) {
            if let Some(el) = Return::dissect(reader)? {
                return next(reader, Element::Return(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Error) {
            if let Some(el) = Error::dissect(reader)? {
                return next(reader, Element::Error(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Compute) {
            if let Some(el) = Compute::dissect(reader)? {
                return next(reader, Element::Compute(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Loop) {
            if let Some(el) = Loop::dissect(reader)? {
                return next(reader, Element::Loop(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::While) {
            if let Some(el) = While::dissect(reader)? {
                return next(reader, Element::While(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::For) {
            if let Some(el) = For::dissect(reader)? {
                return next(reader, Element::For(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Range) {
            if let Some(el) = Range::dissect(reader)? {
                return next(reader, Element::Range(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Breaker) {
            if let Some(el) = Breaker::dissect(reader)? {
                return next(reader, Element::Breaker(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Accessor) {
            if let Some(el) = Accessor::dissect(reader)? {
                return next(reader, Element::Accessor(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Call) {
            if let Some(el) = Call::dissect(reader)? {
                return next(reader, Element::Call(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Optional) {
            if let Some(el) = Optional::dissect(reader)? {
                return next(reader, Element::Optional(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Conclusion) {
            if let Some(el) = Conclusion::dissect(reader)? {
                return next(reader, Element::Conclusion(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Combination) {
            if let Some(el) = Combination::dissect(reader)? {
                return next(reader, Element::Combination(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Subsequence) {
            if let Some(el) = Subsequence::dissect(reader)? {
                return next(reader, Element::Subsequence(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Condition) {
            if let Some(el) = Condition::dissect(reader)? {
                return next(reader, Element::Condition(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Meta) {
            if let Some(el) = Meta::dissect(reader)? {
                return next(reader, Element::Meta(el), token);
            }
        }
        if includes == targets.contains(&ElementId::Comparing) {
            if let Some(el) = Comparing::dissect(reader)? {
                return next(reader, Element::Comparing(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::If) {
            if let Some(el) = If::dissect(reader)? {
                return next(reader, Element::If(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::IfThread) {
            if let Some(el) = IfThread::dissect(reader)? {
                return next(reader, Element::IfThread(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::IfSubsequence) {
            if let Some(el) = IfSubsequence::dissect(reader)? {
                return next(reader, Element::IfSubsequence(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::IfCondition) {
            if let Some(el) = IfCondition::dissect(reader)? {
                return next(reader, Element::IfCondition(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Gatekeeper) {
            if let Some(el) = Gatekeeper::dissect(reader)? {
                return next(reader, Element::Gatekeeper(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Command) {
            if let Some(el) = Command::dissect(reader)? {
                let to = tolerance(reader, md);
                return next(reader, Element::Command(el, to), token);
            }
        }
        if includes == targets.contains(&ElementId::Integer) {
            if let Some(el) = Integer::dissect(reader)? {
                return next(reader, Element::Integer(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Boolean) {
            if let Some(el) = Boolean::dissect(reader)? {
                return next(reader, Element::Boolean(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Incrementer) {
            if let Some(el) = Incrementer::dissect(reader)? {
                return next(reader, Element::Incrementer(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::VariableAssignation) {
            if let Some(el) = VariableAssignation::dissect(reader)? {
                return next(reader, Element::VariableAssignation(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Each) {
            if let Some(el) = Each::dissect(reader)? {
                return next(reader, Element::Each(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::First) {
            if let Some(el) = First::dissect(reader)? {
                return next(reader, Element::First(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Join) {
            if let Some(el) = Join::dissect(reader)? {
                return next(reader, Element::Join(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Function) {
            if let Some(el) = Function::dissect(reader)? {
                let to = tolerance(reader, md);
                return next(reader, Element::Function(el, to), token);
            }
        }
        if includes == targets.contains(&ElementId::Reference) {
            if let Some(el) = Reference::dissect(reader)? {
                return next(reader, Element::Reference(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::PatternString) {
            if let Some(el) = PatternString::dissect(reader)? {
                return next(reader, Element::PatternString(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Block) {
            if let Some(el) = Block::dissect(reader)? {
                return next(reader, Element::Block(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Values) {
            if let Some(el) = Values::dissect(reader)? {
                return next(reader, Element::Values(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::VariableName) {
            if let Some(el) = VariableName::dissect(reader)? {
                return next(reader, Element::VariableName(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Component) {
            if let Some(el) = Component::dissect(reader)? {
                return next(reader, Element::Component(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::Task) {
            if let Some(el) = Task::dissect(reader)? {
                return next(reader, Element::Task(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::VariableDeclaration) {
            if let Some(el) = VariableDeclaration::dissect(reader)? {
                return next(reader, Element::VariableDeclaration(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::VariableType) {
            if let Some(el) = VariableType::dissect(reader)? {
                return next(reader, Element::VariableType(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::VariableVariants) {
            if let Some(el) = VariableVariants::dissect(reader)? {
                return next(reader, Element::VariableVariants(el, md), token);
            }
        }
        if includes == targets.contains(&ElementId::SimpleString) {
            if let Some(el) = SimpleString::dissect(reader)? {
                return next(reader, Element::SimpleString(el, md), token);
            }
        }
        Ok(None)
    }

    pub fn exclude(
        reader: &mut Reader,
        targets: &[ElementId],
    ) -> Result<Option<Element>, LinkedErr<E>> {
        Self::parse(reader, targets, false)
    }

    pub fn include(
        reader: &mut Reader,
        targets: &[ElementId],
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
