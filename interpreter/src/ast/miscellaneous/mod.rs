mod comment;
mod meta;

use crate::*;

impl Interpret for Miscellaneous {
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.interpret(rt),
            Miscellaneous::Meta(n) => n.interpret(rt),
        }
    }
}
