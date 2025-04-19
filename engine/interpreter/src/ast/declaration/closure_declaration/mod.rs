use crate::*;

impl Interpret for ClosureDeclaration {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        Ok(RtValue::Closure(self.uuid))
    }
}
