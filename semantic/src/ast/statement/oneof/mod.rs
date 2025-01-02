use crate::*;

impl InferType for OneOf {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::ExecuteResult.into())
    }
}

impl Initialize for OneOf {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.commands.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for OneOf {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.commands.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}
