#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for Call {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        self.node.interpret(rt).await
    }
}
