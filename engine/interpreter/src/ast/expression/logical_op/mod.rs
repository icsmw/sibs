use crate::*;

impl Interpret for LogicalOp {
    #[boxed]
    fn interpret(&self, _env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::LogicalOperator(self.operator.clone()))
    }
}
