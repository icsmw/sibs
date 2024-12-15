mod comment;
mod include;
mod meta;
mod module;

use crate::*;

impl Interpret for Miscellaneous {
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.interpret(rt),
            Miscellaneous::Include(n) => n.interpret(rt),
            Miscellaneous::Meta(n) => n.interpret(rt),
            Miscellaneous::Module(n) => n.interpret(rt),
        }
    }
}
