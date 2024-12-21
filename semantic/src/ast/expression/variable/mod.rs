use crate::*;

impl InferType for Variable {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let ety = scx
            .tys
            .lookup(&self.ident)
            .ok_or(LinkedErr::token(E::VariableIsNotDefined, &self.token))?;
        if let Some(ty) = ety.assigned.as_ref() {
            if let (Some(negation), false) = (self.negation.as_ref(), matches!(ty, DataType::Bool))
            {
                Err(LinkedErr::between(
                    E::NegationToNotBool,
                    negation,
                    &self.token,
                ))
            } else {
                Ok(ty.to_owned())
            }
        } else {
            Err(LinkedErr::token(E::VariableIsNotDefined, &self.token))
        }
    }
}

impl Initialize for Variable {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .lookup(&self.ident)
            .ok_or(LinkedErr::token(E::VariableIsNotDefined, &self.token))?;
        Ok(())
    }
}
