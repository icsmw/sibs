use crate::*;

// TODO: Now if variable not found, we are dropping errors.
// It would be nice to have keyword `optional` and `required` to specify if variable is required or not. If required, we should return error if not found.
impl Interpret for EnvsModule {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        for node in &self.nodes {
            let Some(name) = node.extract::<VariableName>() else {
                continue;
            };
            if env
                .cx
                .globals()
                .is_registered(&name.ident, &node.link())
                .await
                .map_err(|err| LinkedErr::from(err, node))?
            {
                continue;
            }
            let value = std::env::var(&name.ident).map_err(|err| {
                LinkedErr::from(
                    match err {
                        std::env::VarError::NotPresent => {
                            E::EnvironmentNotDefined(name.ident.clone())
                        }
                        std::env::VarError::NotUnicode(_) => {
                            E::EnvironmentNotUnicode(name.ident.clone())
                        }
                    },
                    node,
                )
            })?;
            env.cx
                .globals()
                .register(
                    name.ident.clone(),
                    DeterminedTy::Str.into(),
                    false,
                    node.link(),
                    RtValue::Str(value),
                )
                .await
                .map_err(|err| LinkedErr::from(err, node))?;
        }
        Ok(RtValue::Void)
    }
}
