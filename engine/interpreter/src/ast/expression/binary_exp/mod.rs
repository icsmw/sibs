use crate::*;

impl Interpret for BinaryExp {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let RtValue::Num(left) = self.left.interpret(env.clone()).await? else {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::Num.to_string()),
                &self.left,
            ));
        };
        let RtValue::Num(right) = self.right.interpret(env.clone()).await? else {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::Num.to_string()),
                &self.right,
            ));
        };
        let RtValue::BinaryOperator(operator) = self.operator.interpret(env.clone()).await? else {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::BinaryOperator.to_string()),
                &self.operator,
            ));
        };
        Ok(RtValue::Num(match operator {
            BinaryOperator::Minus => left - right,
            BinaryOperator::Plus => left + right,
            BinaryOperator::Slash => left / right,
            BinaryOperator::Star => left * right,
        }))
    }
}
