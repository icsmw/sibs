use crate::*;

impl InferType for VariableTypeDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let tys = self
            .types
            .iter()
            .map(|n| n.infer_type(scx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            Err(LinkedErr::token(E::EmptyTypeDeclaration, &self.token))
        } else if tys.len() == 1 {
            Ok(tys[0].clone())
        } else {
            let tys = tys
                .into_iter()
                .map(|ty| {
                    ty.determinated().cloned().ok_or(LinkedErr::token(
                        E::FailInferDeterminedType(ty.clone()),
                        &self.token,
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Ty::OneOf(tys))
        }
    }
}

impl Initialize for VariableTypeDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.types.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for VariableTypeDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.types.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}
