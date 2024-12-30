use crate::*;

impl Interpret for Task {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        self.block.interpret(rt).await
    }
}
