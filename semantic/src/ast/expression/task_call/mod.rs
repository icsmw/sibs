use crate::*;

impl InferType for TaskCall {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for TaskCall {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.initialize(tcx))?;
        Ok(())
    }
}
