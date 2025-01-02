use crate::*;

impl InferType for VariableName {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminatedTy::Void.into())
    }
}

impl Initialize for VariableName {
    fn initialize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

impl Finalization for VariableName {
    fn finalize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
