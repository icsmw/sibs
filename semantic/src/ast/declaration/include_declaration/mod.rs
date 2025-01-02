use crate::*;

impl InferType for IncludeDeclaration {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for IncludeDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)
    }
}

impl Finalization for IncludeDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.finalize(scx)
    }
}
