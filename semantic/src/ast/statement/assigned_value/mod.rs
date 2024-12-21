use crate::*;

impl InferType for AssignedValue {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        self.node.infer_type(scx)
    }
}

impl Initialize for AssignedValue {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}
