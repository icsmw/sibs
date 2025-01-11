use crate::*;

impl InferType for Variable {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let ety = scx
            .tys
            .lookup(&self.ident)
            .map_err(|err| LinkedErr::from(err.into(), self))?
            .ok_or(LinkedErr::from(
                E::VariableIsNotDefined(self.ident.clone()),
                self,
            ))?;
        if let Some(ty) = ety.assigned.as_ref() {
            if let (Some(negation), false) = (self.negation.as_ref(), ty.bool()) {
                Err(LinkedErr::between(
                    E::NegationToNotBool,
                    negation,
                    &self.token,
                ))
            } else {
                Ok(ty.to_owned())
            }
        } else {
            Err(LinkedErr::from(
                E::VariableIsNotDefined(self.ident.clone()),
                self,
            ))
        }
    }
}

impl Initialize for Variable {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .lookup(&self.ident)
            .map_err(|err| LinkedErr::from(err.into(), self))?
            .ok_or(LinkedErr::from(
                E::VariableIsNotDefined(self.ident.clone()),
                self,
            ))?;
        Ok(())
    }
}

impl Finalization for Variable {
    fn finalize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
