use crate::*;

impl Interpret for BinaryExpGroup {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let vl = self.node.interpret(rt, cx.clone()).await?;
        if !matches!(vl, RtValue::Num(..)) {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::Num.to_string()),
                &self.node,
            ));
        };
        Ok(vl)
    }
}
