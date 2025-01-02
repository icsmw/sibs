use crate::*;

impl InferType for Comment {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminatedTy::Void.into())
    }
}

impl Initialize for Comment {
    fn initialize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

impl Finalization for Comment {
    fn finalize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
