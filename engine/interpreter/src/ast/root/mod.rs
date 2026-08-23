mod anchor;
mod component;
mod module;
mod task;

use crate::*;

impl Interpret for Root {
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Root::Task(n) => n.interpret(env),
            Root::Component(n) => n.interpret(env),
            Root::Module(n) => n.interpret(env),
            Root::Anchor(n) => n.interpret(env),
        }
    }
}
