use crate::*;

impl Interpret for Comparison {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}
