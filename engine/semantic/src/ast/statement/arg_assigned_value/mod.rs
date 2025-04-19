use crate::*;

impl InferType for ArgumentAssignedValue {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        self.node.infer_type(scx)
    }
}

impl Initialize for ArgumentAssignedValue {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for ArgumentAssignedValue {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.finalize(scx)
    }
}
