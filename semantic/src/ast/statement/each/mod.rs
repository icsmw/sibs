use crate::*;

impl InferType for Each {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for Each {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.elements.initialize(tcx)?;
        self.element.initialize(tcx)?;
        self.index.initialize(tcx)?;
        self.block.initialize(tcx)
    }
}
