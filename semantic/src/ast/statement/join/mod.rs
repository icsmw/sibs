use crate::*;

impl InferType for Join {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Vec(Box::new(DataType::SpawnStatus)))
    }
}

impl Initialize for Join {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.commands.iter().try_for_each(|n| n.initialize(tcx))?;
        Ok(())
    }
}
