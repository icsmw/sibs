mod assignation;
mod assigned_value;
mod block;
mod r#break;
mod r#for;
mod r#if;
mod join;
mod r#loop;
mod oneof;
mod optional;
mod r#return;
mod r#while;

use crate::*;

impl Interpret for Statement {
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Statement::Assignation(n) => n.interpret(rt),
            Statement::AssignedValue(n) => n.interpret(rt),
            Statement::Block(n) => n.interpret(rt),
            Statement::Break(n) => n.interpret(rt),
            Statement::For(n) => n.interpret(rt),
            Statement::If(n) => n.interpret(rt),
            Statement::Join(n) => n.interpret(rt),
            Statement::Loop(n) => n.interpret(rt),
            Statement::OneOf(n) => n.interpret(rt),
            Statement::Optional(n) => n.interpret(rt),
            Statement::Return(n) => n.interpret(rt),
            Statement::While(n) => n.interpret(rt),
        }
    }
}
