use crate::*;

impl InferType for CompoundAssignmentsOp {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for CompoundAssignmentsOp {
    fn initialize(&self, _tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
