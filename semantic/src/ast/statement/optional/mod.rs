use crate::*;

impl InferType for Optional {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for Optional {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.comparison.initialize(tcx)?;
        self.action.initialize(tcx)
    }
}
