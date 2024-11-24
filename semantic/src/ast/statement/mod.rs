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
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            Statement::Assignation(n) => n.infer_type(tcx),
            Statement::AssignedValue(n) => n.infer_type(tcx),
            Statement::Block(n) => n.infer_type(tcx),
            Statement::Break(n) => n.infer_type(tcx),
            Statement::Each(n) => n.infer_type(tcx),
            Statement::For(n) => n.infer_type(tcx),
            Statement::If(n) => n.infer_type(tcx),
            Statement::Join(n) => n.infer_type(tcx),
            Statement::Loop(n) => n.infer_type(tcx),
            Statement::OneOf(n) => n.infer_type(tcx),
            Statement::Optional(n) => n.infer_type(tcx),
            Statement::Return(n) => n.infer_type(tcx),
            Statement::While(n) => n.infer_type(tcx),
        }
    }
}

impl Initialize for Statement {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        match self {
            Statement::Assignation(n) => n.initialize(tcx),
            Statement::AssignedValue(n) => n.initialize(tcx),
            Statement::Block(n) => n.initialize(tcx),
            Statement::Break(n) => n.initialize(tcx),
            Statement::Each(n) => n.initialize(tcx),
            Statement::For(n) => n.initialize(tcx),
            Statement::If(n) => n.initialize(tcx),
            Statement::Join(n) => n.initialize(tcx),
            Statement::Loop(n) => n.initialize(tcx),
            Statement::OneOf(n) => n.initialize(tcx),
            Statement::Optional(n) => n.initialize(tcx),
            Statement::Return(n) => n.initialize(tcx),
            Statement::While(n) => n.initialize(tcx),
        }
    }
}
