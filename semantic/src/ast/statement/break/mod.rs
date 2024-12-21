use crate::*;

impl InferType for Break {
    fn infer_type(&self, _tcx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for Break {
    fn initialize(&self, _tcx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
