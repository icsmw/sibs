use crate::*;

impl InferType for Gatekeeper {
    fn infer_type(&self, _tcx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for Gatekeeper {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}
