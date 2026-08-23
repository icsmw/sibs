use crate::*;

impl Interpret for BinaryOp {
    #[boxed]
    fn interpret(&self, _env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        Ok(RtValue::BinaryOperator(self.operator.clone()))
    }
}
