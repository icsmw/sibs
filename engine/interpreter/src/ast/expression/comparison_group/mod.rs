use crate::*;

impl Interpret for ComparisonGroup {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        let vl = self.node.interpret(rt, cx).await?;
        if !matches!(vl, RtValue::Bool(..)) {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::Bool.to_string()),
                &self.node,
            ));
        };
        if let (RtValue::Bool(vl), true) = (&vl, self.negation.is_some()) {
            Ok(RtValue::Bool(!*vl))
        } else {
            Ok(vl)
        }
    }
}
