use crate::*;

impl InferType for ModuleDeclaration {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminatedTy::Void.into())
    }
}

impl Initialize for ModuleDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)
    }
}

impl Finalization for ModuleDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.finalize(scx)
    }
}
