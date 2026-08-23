mod arg_assignation;
mod arg_assigned_value;
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
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Statement::Assignation(n) => n.interpret(env),
            Statement::AssignedValue(n) => n.interpret(env),
            Statement::ArgumentAssignation(n) => n.interpret(env),
            Statement::ArgumentAssignedValue(n) => n.interpret(env),
            Statement::Block(n) => n.interpret(env),
            Statement::Break(n) => n.interpret(env),
            Statement::For(n) => n.interpret(env),
            Statement::If(n) => n.interpret(env),
            Statement::Join(n) => n.interpret(env),
            Statement::Loop(n) => n.interpret(env),
            Statement::OneOf(n) => n.interpret(env),
            Statement::Optional(n) => n.interpret(env),
            Statement::Return(n) => n.interpret(env),
            Statement::While(n) => n.interpret(env),
        }
    }
}
