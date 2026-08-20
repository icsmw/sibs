mod comment;
mod meta;

use crate::*;

impl Interpret for Miscellaneous {
    fn interpret(&self, rt: Runtime, cx: ExecutionContext) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.interpret(rt, cx),
            Miscellaneous::Meta(n) => n.interpret(rt, cx),
        }
    }
}
