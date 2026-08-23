use crate::*;

impl Interpret for BinaryExpGroup {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let vl = self.node.interpret(env).await?;
        if !matches!(vl, RtValue::Num(..)) {
            return Err(LinkedErr::from(
                E::InvalidValueType(RtValueId::Num.to_string()),
                &self.node,
            ));
        };
        Ok(vl)
    }
}
