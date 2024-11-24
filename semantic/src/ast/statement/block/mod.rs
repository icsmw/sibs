use crate::*;

impl InferType for Block {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.nodes
            .last()
            .map(|n| n.infer_type(tcx))
            .unwrap_or_else(|| Ok(DataType::Void))
    }
}

impl Initialize for Block {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(tcx))?;
        Ok(())
    }
}
