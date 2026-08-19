use crate::*;

impl Interpret for ComparisonOp {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: Context) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::ComparisonOperator(self.operator.clone()))
    }
}
