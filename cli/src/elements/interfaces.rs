use crate::{
    elements::{Element, Metadata, TokenGetter},
    reader::chars,
};
use std::fmt::{self, Display};

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

#[cfg(test)]
use crate::elements::{ElementId, ElementRefGetter, InnersGetter};

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
    fn get_alias(&self) -> ElementId {
        match self {
            Self::Call(..) => ElementId::Call,
            Self::Accessor(..) => ElementId::Accessor,
            Self::Function(..) => ElementId::Function,
            Self::If(..) => ElementId::If,
            Self::IfCondition(..) => ElementId::IfCondition,
            Self::IfSubsequence(..) => ElementId::IfSubsequence,
            Self::IfThread(..) => ElementId::IfThread,
            Self::Breaker(..) => ElementId::Breaker,
            Self::Each(..) => ElementId::Each,
            Self::First(..) => ElementId::First,
            Self::Join(..) => ElementId::Join,
            Self::VariableAssignation(..) => ElementId::VariableAssignation,
            Self::Comparing(..) => ElementId::Comparing,
            Self::Combination(..) => ElementId::Combination,
            Self::Condition(..) => ElementId::Condition,
            Self::Subsequence(..) => ElementId::Subsequence,
            Self::Optional(..) => ElementId::Optional,
            Self::Gatekeeper(..) => ElementId::Gatekeeper,
            Self::Reference(..) => ElementId::Reference,
            Self::PatternString(..) => ElementId::PatternString,
            Self::VariableName(..) => ElementId::VariableName,
            Self::Values(..) => ElementId::Values,
            Self::Meta(..) => ElementId::Meta,
            Self::Block(..) => ElementId::Block,
            Self::Command(..) => ElementId::Command,
            Self::Task(..) => ElementId::Task,
            Self::Component(..) => ElementId::Component,
            Self::Boolean(..) => ElementId::Boolean,
            Self::Integer(..) => ElementId::Integer,
            Self::VariableDeclaration(..) => ElementId::VariableDeclaration,
            Self::VariableVariants(..) => ElementId::VariableVariants,
            Self::VariableType(..) => ElementId::VariableType,
            Self::SimpleString(..) => ElementId::SimpleString,
            Self::Range(..) => ElementId::Range,
            Self::For(..) => ElementId::For,
            Self::Compute(..) => ElementId::Compute,
            Self::Return(..) => ElementId::Return,
            Self::Error(..) => ElementId::Error,
            Self::Incrementer(..) => ElementId::Incrementer,
            Self::Loop(..) => ElementId::Loop,
            Self::While(..) => ElementId::While,
            Self::Closure(..) => ElementId::Closure,
            Self::Conclusion(..) => ElementId::Conclusion,
            Self::Comment(..) => ElementId::Comment,
        }
    }
}
