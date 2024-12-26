use crate::*;

impl Interpret for BinaryOp {
    #[boxed]
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        Ok(RtValue::BinaryOperator(self.operator.clone()))
    }
}
