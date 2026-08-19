use crate::*;

impl Interpret for CompoundAssignmentsOp {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: Context) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}
