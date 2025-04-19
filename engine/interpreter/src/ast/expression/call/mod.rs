#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Call {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        self.node.interpret(rt, cx).await
    }
}
