use crate::*;

impl Interpret for BinaryExpGroup {
    #[boxed]
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        let vl = self.node.interpret(rt).await?;
        if !matches!(vl, RtValue::Num(..)) {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::Num.to_string()),
                &self.node,
            ));
        };
        Ok(vl)
    }
}
