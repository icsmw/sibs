use crate::*;

impl InferType for VariableName {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for VariableName {
    fn initialize(&self, _tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
