use crate::*;

impl InferType for LogicalOp {
    fn infer_type(&self, _tcx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for LogicalOp {
    fn initialize(&self, _tcx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
