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
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Statement::Assignation(n) => n.interpret(rt, cx),
            Statement::AssignedValue(n) => n.interpret(rt, cx),
            Statement::Block(n) => n.interpret(rt, cx),
            Statement::Break(n) => n.interpret(rt, cx),
            Statement::For(n) => n.interpret(rt, cx),
            Statement::If(n) => n.interpret(rt, cx),
            Statement::Join(n) => n.interpret(rt, cx),
            Statement::Loop(n) => n.interpret(rt, cx),
            Statement::OneOf(n) => n.interpret(rt, cx),
            Statement::Optional(n) => n.interpret(rt, cx),
            Statement::Return(n) => n.interpret(rt, cx),
            Statement::While(n) => n.interpret(rt, cx),
        }
    }
}
