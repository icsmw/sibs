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
