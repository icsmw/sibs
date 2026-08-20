use crate::*;

impl Interpret for ArgumentDeclaration {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: ExecutionContext) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}
