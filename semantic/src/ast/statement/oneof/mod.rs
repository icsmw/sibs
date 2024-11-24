use crate::*;

impl InferType for OneOf {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::SpawnStatus)
    }
}

impl Initialize for OneOf {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.commands.iter().try_for_each(|n| n.initialize(tcx))?;
        Ok(())
    }
}
