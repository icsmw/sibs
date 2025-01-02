use crate::*;

impl InferType for Break {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminatedTy::Void.into())
    }
}

impl Initialize for Break {
    fn initialize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

impl Finalization for Break {
    fn finalize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
