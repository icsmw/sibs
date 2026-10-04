mod anchor;
mod component;
mod envs;
mod globals;
mod module;
mod task;

use crate::*;

impl Interpret for Root {
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Root::GlobalsImport(n) => n.interpret(env),
            Root::EnvsImport(n) => n.interpret(env),
            Root::GlobalsModule(n) => n.interpret(env),
            Root::EnvsModule(n) => n.interpret(env),
            Root::Task(n) => n.interpret(env),
            Root::Component(n) => n.interpret(env),
            Root::Module(n) => n.interpret(env),
            Root::Anchor(n) => n.interpret(env),
        }
    }
}
