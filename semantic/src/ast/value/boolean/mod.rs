use crate::*;

impl InferType for Boolean {
    fn infer_type(&self, _tcx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Bool)
    }
}

impl Initialize for Boolean {
    fn initialize(&self, _tcx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
