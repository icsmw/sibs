use crate::*;

impl InferType for Anchor {
    fn infer_type(&self, _tcx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
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
