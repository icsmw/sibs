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

impl InferType for Statement {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        match self {
            Statement::Assignation(n) => n.infer_type(scx),
            Statement::AssignedValue(n) => n.infer_type(scx),
            Statement::Block(n) => n.infer_type(scx),
            Statement::Break(n) => n.infer_type(scx),
            Statement::Each(n) => n.infer_type(scx),
            Statement::For(n) => n.infer_type(scx),
            Statement::If(n) => n.infer_type(scx),
            Statement::Join(n) => n.infer_type(scx),
            Statement::Loop(n) => n.infer_type(scx),
            Statement::OneOf(n) => n.infer_type(scx),
            Statement::Optional(n) => n.infer_type(scx),
            Statement::Return(n) => n.infer_type(scx),
            Statement::While(n) => n.infer_type(scx),
        }
    }
}

impl Initialize for Statement {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Statement::Assignation(n) => n.initialize(scx),
            Statement::AssignedValue(n) => n.initialize(scx),
            Statement::Block(n) => n.initialize(scx),
            Statement::Break(n) => n.initialize(scx),
            Statement::Each(n) => n.initialize(scx),
            Statement::For(n) => n.initialize(scx),
            Statement::If(n) => n.initialize(scx),
            Statement::Join(n) => n.initialize(scx),
            Statement::Loop(n) => n.initialize(scx),
            Statement::OneOf(n) => n.initialize(scx),
            Statement::Optional(n) => n.initialize(scx),
            Statement::Return(n) => n.initialize(scx),
            Statement::While(n) => n.initialize(scx),
        }
    }
}
