use crate::*;

impl Interpret for EnvDeclaration {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let name = self.variable.extract::<VariableName>().ok_or_else(|| {
            LinkedErr::from(
                E::UnexpectedNode(self.variable.get_node().id()),
                &self.variable,
            )
        })?;
        let globals = env.cx.globals();
        if globals
            .is_registered(&name.ident, &self.variable.link())
            .await
            .map_err(|err| LinkedErr::from(err, &self.variable))?
        {
            return Ok(RtValue::Void);
        }
        let value = match std::env::var(&name.ident) {
            Ok(value) => value,
            Err(std::env::VarError::NotPresent) if self.is_optional() => String::new(),
            Err(std::env::VarError::NotPresent) => {
                return Err(LinkedErr::from(
                    E::EnvironmentNotDefined(name.ident.clone()),
                    &self.variable,
                ));
            }
            Err(std::env::VarError::NotUnicode(_)) => {
                return Err(LinkedErr::from(
                    E::EnvironmentNotUnicode(name.ident.clone()),
                    &self.variable,
                ));
            }
        };
        globals
            .register(
                name.ident.clone(),
                DeterminedTy::Str.into(),
                false,
                self.variable.link(),
                RtValue::Str(value),
            )
            .await
            .map_err(|err| LinkedErr::from(err, &self.variable))?;
        Ok(RtValue::Void)
    }
}
