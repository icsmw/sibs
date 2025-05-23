use crate::*;

impl InferType for Optional {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for Optional {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.comparison.initialize(scx)?;
        self.action.initialize(scx)
    }
}

impl Finalization for Optional {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.comparison.finalize(scx)?;
        self.action.finalize(scx)
    }
}
