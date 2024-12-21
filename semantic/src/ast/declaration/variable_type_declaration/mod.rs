use crate::*;

impl InferType for VariableTypeDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let tys = self
            .types
            .iter()
            .map(|n| n.infer_type(scx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            Err(LinkedErr::unlinked(E::EmptyTypeDeclaration))
        } else if tys.len() == 1 {
            Ok(tys[0].clone())
        } else {
            Ok(DataType::OneOf(tys))
        }
    }
}

impl Initialize for VariableTypeDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.types.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}
