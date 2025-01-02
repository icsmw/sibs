use crate::*;

impl InferType for BinaryOp {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminatedTy::Void.into())
    }
}

impl Initialize for BinaryOp {
    fn initialize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

impl Finalization for BinaryOp {
    fn finalize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
