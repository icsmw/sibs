use crate::*;

impl InferType for While {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for While {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.comparison.initialize(scx)?;
        self.block.initialize(scx)
    }
}

impl Finalization for While {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.comparison.finalize(scx)?;
        self.block.finalize(scx)
    }
}
