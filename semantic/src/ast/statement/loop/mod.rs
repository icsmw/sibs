use crate::*;

impl InferType for Loop {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for Loop {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.block.initialize(scx)
    }
}

impl Finalization for Loop {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.block.finalize(scx)
    }
}
