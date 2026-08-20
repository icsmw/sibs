use crate::*;

impl Interpret for ClosureDeclaration {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: ExecutionContext) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::Closure(self.uuid))
    }
}
