use crate::*;

impl InferType for Task {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        scx.tys.open(&self.uuid);
        let ty = self.block.infer_type(scx)?;
        scx.tys
            .close()
            .map_err(|err| LinkedErr::token(err, &self.name))?;
        Ok(ty)
    }
}

impl Initialize for Task {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys.open(&self.uuid);
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        self.block.initialize(scx)?;
        scx.tys
            .close()
            .map_err(|err| LinkedErr::token(err, &self.name))?;
        Ok(())
    }
}

impl Finalization for Task {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys.open(&self.uuid);
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        self.block.finalize(scx)?;
        scx.tys
            .close()
            .map_err(|err| LinkedErr::token(err, &self.name))?;
        Ok(())
    }
}
