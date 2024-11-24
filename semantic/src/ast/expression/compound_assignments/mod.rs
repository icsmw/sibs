use crate::*;

impl InferType for CompoundAssignments {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for CompoundAssignments {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.left.initialize(tcx)?;
        self.operator.initialize(tcx)?;
        self.right.initialize(tcx)
    }
}
