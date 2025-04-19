use crate::*;

impl Interpret for VariableName {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}
