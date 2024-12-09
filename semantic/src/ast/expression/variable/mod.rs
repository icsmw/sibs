use crate::*;

impl InferType for Variable {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let ety = tcx
            .lookup(&self.ident)
            .ok_or(LinkedErr::token(E::VariableIsNotDefined, &self.token))?;
        if let Some(ty) = ety.assigned.as_ref() {
            Ok(ty.to_owned())
        } else {
            Err(LinkedErr::token(E::VariableIsNotDefined, &self.token))
        }
    }
}

impl Initialize for Variable {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        tcx.lookup(&self.ident)
            .ok_or(LinkedErr::token(E::VariableIsNotDefined, &self.token))?;
        Ok(())
    }
}
