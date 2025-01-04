use crate::*;

impl Interpret for ClosureDeclaration {
    #[boxed]
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        Ok(RtValue::Closure(self.uuid))
    }
}
