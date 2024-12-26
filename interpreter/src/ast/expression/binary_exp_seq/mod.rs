#[cfg(test)]
mod tests;

use crate::*;

impl Interpret for BinaryExpSeq {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let mut assemble: f64 = 0.0;
        let mut operator = None;
        for (n, next) in self.nodes.iter().enumerate() {
            if n == 0 {
                let RtValue::Num(vl) = next.interpret(rt.clone()).await? else {
                    return Err(LinkedErr::by_node(
                        E::InvalidValueType(RtValueId::Num.to_string()),
                        next,
                    ));
                };
                assemble = vl;
            } else if n % 2 != 0 {
                if operator.is_some() {
                    return Err(LinkedErr::by_node(E::MissedBinaryOperator, next));
                }
                let RtValue::BinaryOperator(op) = next.interpret(rt.clone()).await? else {
                    return Err(LinkedErr::by_node(
                        E::InvalidValueType(RtValueId::BinaryOperator.to_string()),
                        next,
                    ));
                };
                operator = Some(op);
            } else if n % 2 == 0 {
                let Some(operator) = operator.take() else {
                    return Err(LinkedErr::by_node(E::MissedBinaryOperator, next));
                };
                let RtValue::Num(vl) = next.interpret(rt.clone()).await? else {
                    return Err(LinkedErr::by_node(
                        E::InvalidValueType(RtValueId::Num.to_string()),
                        next,
                    ));
                };
                assemble = match operator {
                    BinaryOperator::Minus => assemble - vl,
                    BinaryOperator::Plus => assemble + vl,
                    BinaryOperator::Slash => assemble / vl,
                    BinaryOperator::Star => assemble * vl,
                };
            }
        }
        Ok(RtValue::Num(assemble))
    }
}
