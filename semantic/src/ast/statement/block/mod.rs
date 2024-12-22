use crate::*;

impl InferType for Block {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        scx.tys.enter(&self.uuid);
        let ty = self
            .nodes
            .last()
            .map(|n| n.infer_type(scx))
            .unwrap_or_else(|| Ok(DataType::Void))?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::between(err, &self.open, &self.close))?;
        Ok(ty)
    }
}

impl Initialize for Block {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys.enter(&self.uuid);
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::between(err, &self.open, &self.close))?;
        Ok(())
    }
}

impl Finalization for Block {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys.enter(&self.uuid);
        self.nodes.iter().try_for_each(|n| n.finalize(scx))?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::between(err, &self.open, &self.close))?;
        Ok(())
    }
}
