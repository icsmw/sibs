use crate::*;

impl InferType for Anchor {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for Anchor {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        for node in self.nodes.iter() {
            node.initialize(tcx)?;
        }
        self.infer_type(tcx).map(|_| ())
    }
}
