use crate::*;

impl InferType for Component {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for Component {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tasks.master(self.get_name(), &self.uuid);
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for Component {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tasks.master(self.get_name(), &self.uuid);
        self.nodes.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}
