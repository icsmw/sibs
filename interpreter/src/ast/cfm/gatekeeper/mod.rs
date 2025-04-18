use crate::*;

impl Interpret for Gatekeeper {
    #[boxed]
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        for node in self.nodes.iter() {
            let value = node.interpret(rt.clone(), cx.clone()).await?;
            let RtValue::Bool(proceed) = value else {
                return Err(LinkedErr::from(
                    E::InvalidType(Ty::Determined(DeterminedTy::Bool), value),
                    node,
                ));
            };
            if !proceed {
                return Ok(RtValue::Bool(false));
            }
        }
        Ok(RtValue::Bool(true))
    }
}
