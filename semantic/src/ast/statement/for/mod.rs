use crate::*;

impl InferType for For {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for For {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.elements.initialize(scx)?;
        self.element.initialize(scx)?;
        self.index.initialize(scx)?;
        self.block.initialize(scx)
    }
}
