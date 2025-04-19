mod anchor;
mod component;
mod module;
mod task;

use crate::*;

impl Interpret for Root {
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Root::Task(n) => n.interpret(rt, cx),
            Root::Component(n) => n.interpret(rt, cx),
            Root::Module(n) => n.interpret(rt, cx),
            Root::Anchor(n) => n.interpret(rt, cx),
        }
    }
}
