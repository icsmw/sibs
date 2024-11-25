use crate::*;

impl InferType for VariableTypeDeclaration {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let tys = self
            .types
            .iter()
            .map(|n| n.infer_type(tcx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            Err(LinkedErr::by_link(E::EmptyTypeDeclaration, &self.into()))
        } else if tys.len() == 1 {
            Ok(tys[0].clone())
        } else {
            Ok(DataType::OneOf(tys))
        }
    }
}

impl Initialize for VariableTypeDeclaration {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.types.iter().try_for_each(|n| n.initialize(tcx))?;
        Ok(())
    }
}
