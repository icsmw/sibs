use crate::*;

impl Interpret for Block {
    #[boxed]
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}
