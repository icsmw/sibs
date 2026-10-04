use crate::*;

impl Interpret for GlobalDeclaration {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let variable = self.variable.extract::<VariableName>().ok_or_else(|| {
            LinkedErr::from(
                E::UnexpectedNode(self.variable.get_node().id()),
                &self.variable,
            )
        })?;
        let globals = env.cx.globals();
        if globals
            .is_registered(&variable.ident, &self.variable.link())
            .await
            .map_err(|err| LinkedErr::from(err, &self.variable))?
        {
            return Ok(RtValue::Void);
        }
        let ty = env
            .rt
            .tys
            .get(self.variable.uuid())
            .ok_or_else(|| LinkedErr::from(E::FailInferType, &self.variable))?;
        let value = self.assignation.interpret(env.clone()).await?;
        globals
            .register(
                variable.ident.clone(),
                ty,
                self.is_mutable(),
                self.variable.link(),
                value,
            )
            .await
            .map_err(|err| LinkedErr::from(err, &self.assignation))?;
        Ok(RtValue::Void)
    }
}
