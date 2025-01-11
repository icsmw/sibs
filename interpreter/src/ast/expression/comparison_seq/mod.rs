use crate::*;

impl Interpret for ComparisonSeq {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let Some(node) = self.nodes.first() else {
            return Err(LinkedErr::from(E::InvalidComparisonSeq, self));
        };
        let RtValue::Bool(mut result) = node.interpret(rt.clone()).await? else {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::Bool.to_string()),
                node,
            ));
        };
        let mut prev_op = None;
        for n in self.nodes.iter().skip(1) {
            let vl = n.interpret(rt.clone()).await?;
            match vl {
                RtValue::Bool(vl) => {
                    let Some(op) = prev_op.take() else {
                        return Err(LinkedErr::from(
                            E::InvalidValueType(RtValueId::Bool.to_string()),
                            n,
                        ));
                    };
                    match op {
                        LogicalOperator::Or => result = result || vl,
                        LogicalOperator::And => result = result && vl,
                    }
                }
                RtValue::LogicalOperator(op) => prev_op = Some(op),
                _ => {
                    return Err(LinkedErr::from(
                        E::InvalidValueType(RtValueId::Bool.to_string()),
                        n,
                    ));
                }
            }
        }
        Ok(RtValue::Bool(result))
    }
}
