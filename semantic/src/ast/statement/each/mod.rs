use crate::*;

impl InferType for Each {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminatedTy::Void.into())
    }
}

impl Initialize for Each {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.elements.initialize(scx)?;
        self.element.initialize(scx)?;
        self.index.initialize(scx)?;
        self.block.initialize(scx)
    }
}

impl Finalization for Each {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.elements.finalize(scx)?;
        self.element.finalize(scx)?;
        self.index.finalize(scx)?;
        self.block.finalize(scx)
    }
}
