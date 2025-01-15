#[cfg(test)]
mod tests;

use crate::*;

impl InferType for While {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for While {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        self.comparison.initialize(scx)?;
        let ty = self.comparison.infer_type(scx)?;
        if !matches!(ty, Ty::Determined(DeterminedTy::Bool)) {
            return Err(LinkedErr::from(
                E::DismatchTypes(format!("{} and {ty}", Ty::Determined(DeterminedTy::Bool))),
                &self.comparison,
            ));
        }
        self.block.initialize(scx)?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        Ok(())
    }
}

impl Finalization for While {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        self.comparison.finalize(scx)?;
        self.block.finalize(scx)?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        Ok(())
    }
}
