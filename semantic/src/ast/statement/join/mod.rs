use crate::*;

impl InferType for Join {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Vec(Box::new(DataType::ExecuteResult)))
    }
}

impl Initialize for Join {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.commands.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for Join {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.commands.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}
