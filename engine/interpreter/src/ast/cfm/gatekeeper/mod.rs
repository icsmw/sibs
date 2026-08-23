use crate::*;

impl Interpret for Gatekeeper {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        for node in self.nodes.iter() {
            let value = node.interpret(env.clone()).await?;
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
