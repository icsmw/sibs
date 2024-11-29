use crate::*;

impl InferType for Variable {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(tcx
            .lookup(&self.ident)
            .ok_or(LinkedErr::by_link(E::VariableIsNotDefined, &self.into()))?
            .clone())
    }
}

impl Initialize for Variable {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        tcx.lookup(&self.ident)
            .ok_or(LinkedErr::by_link(E::VariableIsNotDefined, &self.into()))?;
        Ok(())
    }
}
