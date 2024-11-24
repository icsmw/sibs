use crate::*;

impl InferType for Number {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::F64)
    }
}

impl Initialize for Number {
    fn initialize(&self, _tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
