use crate::*;

impl Interpret for ArgumentDeclaration {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: Context) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}
