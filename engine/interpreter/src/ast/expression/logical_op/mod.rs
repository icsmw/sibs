use crate::*;

impl Interpret for LogicalOp {
    #[boxed]
    fn interpret(&self, _rt: Runtime, _cx: ExecutionContext) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::LogicalOperator(self.operator.clone()))
    }
}
