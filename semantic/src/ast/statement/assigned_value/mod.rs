use crate::*;

impl InferType for AssignedValue {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.node.infer_type(tcx)
    }
}

impl Initialize for AssignedValue {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.node.initialize(tcx)?;
        self.infer_type(tcx).map(|_| ())
    }
}
