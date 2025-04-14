use crate::*;

impl Interpret for BinaryExp {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let RtValue::Num(left) = self.left.interpret(rt.clone(), cx.clone()).await? else {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::Num.to_string()),
                &self.left,
            ));
        };
        let RtValue::Num(right) = self.right.interpret(rt.clone(), cx.clone()).await? else {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::Num.to_string()),
                &self.right,
            ));
        };
        let RtValue::BinaryOperator(operator) = self.operator.interpret(rt, cx.clone()).await?
        else {
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
