mod component;
mod task;

use crate::*;

impl Interpret for Root {
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Root::Task(n) => n.interpret(rt),
            Root::Component(n) => n.interpret(rt),
        }
    }
}
