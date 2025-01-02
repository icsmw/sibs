use crate::*;

impl InferType for Return {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        self.node
            .as_ref()
            .map(|n| n.infer_type(scx))
            .unwrap_or_else(|| Ok(DeterminatedTy::Void.into()))
    }
}

impl Initialize for Return {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let Some(n) = self.node.as_ref() {
            n.initialize(scx)?;
            n.infer_type(scx).map(|_| ())?;
        }
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Return {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let Some(n) = self.node.as_ref() {
            n.finalize(scx)?;
        }
        Ok(())
    }
}
