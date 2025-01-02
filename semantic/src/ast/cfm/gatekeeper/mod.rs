use crate::*;

impl InferType for Gatekeeper {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for Gatekeeper {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for Gatekeeper {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}
