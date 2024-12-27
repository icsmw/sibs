use crate::*;

impl Interpret for Comparison {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let RtValue::ComparisonOperator(op) = self.operator.interpret(rt.clone()).await? else {
            return Err(LinkedErr::by_node(
                E::InvalidValueType(RtValueId::ComparisonOperator.to_string()),
                &self.operator,
            ));
        };
        let left = self
            .left
            .interpret(rt.clone())
            .await?
            .into_eq_ord()
            .ok_or(LinkedErr::by_node(E::NotComparableValue, &self.left))?;
        let right = self
            .right
            .interpret(rt.clone())
            .await?
            .into_eq_ord()
            .ok_or(LinkedErr::by_node(E::NotComparableValue, &self.right))?;
        if left.id() != right.id() {
            return Err(LinkedErr::between_nodes(
                E::DifferentTypeOfValues,
                &self.left,
                &self.right,
            ));
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
