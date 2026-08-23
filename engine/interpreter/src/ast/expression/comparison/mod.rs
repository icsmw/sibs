use crate::*;

impl Interpret for Comparison {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let RtValue::ComparisonOperator(op) = self.operator.interpret(env.clone()).await? else {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::ComparisonOperator.to_string()),
                &self.operator,
            ));
        };
        let left = self
            .left
            .interpret(env.clone())
            .await?
            .into_eq_ord()
            .ok_or(LinkedErr::from(E::NotComparableValue, &self.left))?;
        let right = self
            .right
            .interpret(env.clone())
            .await?
            .into_eq_ord()
            .ok_or(LinkedErr::from(E::NotComparableValue, &self.right))?;
        if left.id() != right.id() {
            return Err(LinkedErr::from(E::DifferentTypeOfValues, self));
        }
        Ok(RtValue::Bool(match op {
            ComparisonOperator::BangEqual => left != right,
            ComparisonOperator::EqualEqual => left == right,
            ComparisonOperator::Greater => left > right,
            ComparisonOperator::GreaterEqual => left >= right,
            ComparisonOperator::Less => left < right,
            ComparisonOperator::LessEqual => left <= right,
        }))
    }
}
