use crate::*;

impl Interpret for Error {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: ExecutionContext) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}
