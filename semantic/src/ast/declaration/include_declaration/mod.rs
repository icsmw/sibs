use crate::*;

impl InferType for IncludeDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        self.root.infer_type(scx)
    }
}

impl Initialize for IncludeDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)?;
        self.root.initialize(scx)
    }
}

impl Finalization for IncludeDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.finalize(scx)?;
        self.root.finalize(scx)
    }
}
