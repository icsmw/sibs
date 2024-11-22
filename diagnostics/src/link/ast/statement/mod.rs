mod assignation;
mod assigned_value;
mod block;
mod r#break;
mod each;
mod r#for;
mod r#if;
mod join;
mod r#loop;
mod oneof;
mod optional;
mod r#return;
mod r#while;

use crate::*;
use asttree::*;

impl From<&Statement> for SrcLink {
    fn from(node: &Statement) -> Self {
        match node {
            Statement::Block(n) => n.into(),
            Statement::Break(n) => n.into(),
            Statement::Return(n) => n.into(),
            Statement::Optional(n) => n.into(),
            Statement::If(n) => n.into(),
            Statement::For(n) => n.into(),
            Statement::While(n) => n.into(),
            Statement::Loop(n) => n.into(),
            Statement::Each(n) => n.into(),
            Statement::Assignation(n) => n.into(),
            Statement::AssignedValue(n) => n.into(),
            Statement::OneOf(n) => n.into(),
            Statement::Join(n) => n.into(),
        }
    }
}
