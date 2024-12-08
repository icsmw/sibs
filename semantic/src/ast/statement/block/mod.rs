use crate::*;

impl InferType for Block {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        tcx.enter(&self.uuid);
        let ty = self
            .nodes
            .last()
            .map(|n| n.infer_type(tcx))
            .unwrap_or_else(|| Ok(DataType::Void))?;
        tcx.leave()
            .map_err(|err| LinkedErr::between(err, &self.open, &self.close))?;
        Ok(ty)
    }
}

impl Initialize for Block {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        tcx.enter(&self.uuid);
        self.nodes.iter().try_for_each(|n| n.initialize(tcx))?;
        tcx.leave()
            .map_err(|err| LinkedErr::between(err, &self.open, &self.close))?;
        Ok(())
    }
}
