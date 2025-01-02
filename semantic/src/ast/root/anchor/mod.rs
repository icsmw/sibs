use crate::*;

impl InferType for Anchor {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for Anchor {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        for node in self.nodes.iter() {
            node.initialize(scx)?;
        }
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Anchor {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        for node in self.nodes.iter() {
            node.finalize(scx)?;
        }
        Ok(())
    }
}
