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
    fn try_read(
        reader: &mut Reader,
        target: ElementId,
        md: Metadata,
    ) -> Result<Option<Element>, LinkedErr<E>> {
        let el = match target {
            ElementId::Call => Call::dissect(reader)?.map(|el| Element::Call(el, md)),
            ElementId::Accessor => Accessor::dissect(reader)?.map(|el| Element::Accessor(el, md)),
            ElementId::Function => Function::dissect(reader)?.map(|el| Element::Function(el, md)),
            ElementId::If => If::dissect(reader)?.map(|el| Element::If(el, md)),
            ElementId::IfCondition => {
                IfCondition::dissect(reader)?.map(|el| Element::IfCondition(el, md))
            }
            ElementId::IfSubsequence => {
                IfSubsequence::dissect(reader)?.map(|el| Element::IfSubsequence(el, md))
            }
            ElementId::IfThread => IfThread::dissect(reader)?.map(|el| Element::IfThread(el, md)),
            ElementId::Each => Each::dissect(reader)?.map(|el| Element::Each(el, md)),
            ElementId::Breaker => Breaker::dissect(reader)?.map(|el| Element::Breaker(el, md)),
            ElementId::First => First::dissect(reader)?.map(|el| Element::First(el, md)),
            ElementId::Join => Join::dissect(reader)?.map(|el| Element::Join(el, md)),
            ElementId::VariableAssignation => {
                VariableAssignation::dissect(reader)?.map(|el| Element::VariableAssignation(el, md))
            }
            ElementId::Optional => Optional::dissect(reader)?.map(|el| Element::Optional(el, md)),
            ElementId::Gatekeeper => {
                Gatekeeper::dissect(reader)?.map(|el| Element::Gatekeeper(el, md))
            }
            ElementId::Reference => {
                Reference::dissect(reader)?.map(|el| Element::Reference(el, md))
            }
            ElementId::PatternString => {
                PatternString::dissect(reader)?.map(|el| Element::PatternString(el, md))
            }
            ElementId::VariableName => {
                VariableName::dissect(reader)?.map(|el| Element::VariableName(el, md))
            }
            ElementId::Comparing => {
                Comparing::dissect(reader)?.map(|el| Element::Comparing(el, md))
            }
            ElementId::Combination => {
                Combination::dissect(reader)?.map(|el| Element::Combination(el, md))
            }
            ElementId::Subsequence => {
                Subsequence::dissect(reader)?.map(|el| Element::Subsequence(el, md))
            }
            ElementId::Condition => {
                Condition::dissect(reader)?.map(|el| Element::Condition(el, md))
            }
            ElementId::Values => Values::dissect(reader)?.map(|el| Element::Values(el, md)),
            ElementId::Block => Block::dissect(reader)?.map(|el| Element::Block(el, md)),
            ElementId::Meta => Meta::dissect(reader)?.map(Element::Meta),
            ElementId::Command => Command::dissect(reader)?.map(|el| Element::Command(el, md)),
            ElementId::Task => Task::dissect(reader)?.map(|el| Element::Task(el, md)),
            ElementId::Component => {
                Component::dissect(reader)?.map(|el| Element::Component(el, md))
            }
            ElementId::Integer => Integer::dissect(reader)?.map(|el| Element::Integer(el, md)),
            ElementId::Boolean => Boolean::dissect(reader)?.map(|el| Element::Boolean(el, md)),
            ElementId::VariableDeclaration => {
                VariableDeclaration::dissect(reader)?.map(|el| Element::VariableDeclaration(el, md))
            }
            ElementId::VariableVariants => {
                VariableVariants::dissect(reader)?.map(|el| Element::VariableVariants(el, md))
            }
            ElementId::VariableType => {
                VariableType::dissect(reader)?.map(|el| Element::VariableType(el, md))
            }
            ElementId::SimpleString => {
                SimpleString::dissect(reader)?.map(|el| Element::SimpleString(el, md))
            }
            ElementId::Range => Range::dissect(reader)?.map(|el| Element::Range(el, md)),
            ElementId::For => For::dissect(reader)?.map(|el| Element::For(el, md)),
            ElementId::Return => Return::dissect(reader)?.map(|el| Element::Return(el, md)),
            ElementId::Error => Error::dissect(reader)?.map(|el| Element::Error(el, md)),
            ElementId::Compute => Compute::dissect(reader)?.map(|el| Element::Compute(el, md)),
            ElementId::Incrementer => {
                Incrementer::dissect(reader)?.map(|el| Element::Incrementer(el, md))
            }
            ElementId::Loop => Loop::dissect(reader)?.map(|el| Element::Loop(el, md)),
            ElementId::While => While::dissect(reader)?.map(|el| Element::While(el, md)),
            ElementId::Closure => Closure::dissect(reader)?.map(|el| Element::Closure(el, md)),
            ElementId::Conclusion => {
                Conclusion::dissect(reader)?.map(|el| Element::Conclusion(el, md))
            }
            ElementId::Comment => Comment::dissect(reader)?.map(Element::Comment),
        };
        Ok(el)
    }

    pub fn read(
        reader: &mut Reader,
        targets: &[ElementId],
    ) -> Result<Option<Element>, LinkedErr<E>> {
        fn next(
            reader: &mut Reader,
            mut el: Element,
            token: impl Fn(&mut Reader) -> usize,
        ) -> Result<Option<Element>, LinkedErr<E>> {
            let Some(ppm) = Element::read(reader, &[ElementId::Call, ElementId::Accessor])? else {
                el.get_mut_metadata().set_token(token(reader));
                return Ok(Some(el));
            };
            el.get_mut_metadata().set_ppm(ppm).set_token(token(reader));
            Ok(Some(el))
        }
        let drop_pos = reader.pin();
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
        let md: Metadata = Metadata {
            comments,
            meta,
            ppm: None,
            tolerance: false,
            inverting: reader.move_to().char(&[&chars::EXCLAMATION]).is_some(),
            token: 0,
        };
        let mut done = Vec::new();
        let mut fail = Vec::new();
        for target in targets.sort_by_uniqueness().iter() {
            let drop_pos = reader.pin();
            let result = Element::try_read(reader, **target, md.clone());
            let restore_point = drop_pos(reader);
            match result {
                Ok(Some(el)) => {
                    done.push((restore_point.0, (el, restore_point)));
                    if target.is_sufficient() {
                        break;
                    }
                }
                Ok(None) => {}
                Err(err) => {
                    fail.push((restore_point.0, (err, restore_point)));
                }
            }
        }
        if let (Some((_, (err, restore_point))), true) = (
            fail.into_iter().max_by_key(|(pos, _)| *pos),
            done.is_empty(),
        ) {
            reader.restore_to_point(restore_point);
            return Err(err);
        }
        let Some((n, _)) = done.iter().enumerate().max_by_key(|(_, (pos, _))| pos) else {
            drop_pos(reader);
            return Ok(None);
        };
        let (pos, (el, restore_point)) = done.remove(n);
        reader.restore_to_point(restore_point);
        let mut el = if let Some((sec_n, (sec_el, _))) = done.into_iter().find(|(p, _)| p == &pos) {
            if let Some(resolved) = el.id().resolve_conflict(sec_el.id()) {
                if el.id() == resolved {
                    el
                } else {
                    sec_el
                }
            } else {
                println!(">>>>>>>>>>>>>>>>>>>>:{}", reader.content);
                println!(">>>>>>>>>>>>>>>>>>>>({pos}):{el:?}");
                println!(">>>>>>>>>>>>>>>>>>>>({sec_n}):{sec_el:?}");
                return Err(E::ConflictBetweenElements(el.id(), sec_el.id()).by_reader(reader));
            }
        } else {
            el
        };
        // let mut el = Element::try_read(reader, el.id(), md.clone())?.unwrap();
        el.get_mut_metadata().tolerance = reader.move_to().char(&[&chars::QUESTION]).is_some();
        next(reader, el, token)
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
