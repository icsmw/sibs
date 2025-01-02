use crate::*;

impl InferType for ComparisonOp {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminatedTy::Void.into())
    }
}

impl Initialize for ComparisonOp {
    fn initialize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

impl Finalization for ComparisonOp {
    fn finalize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
