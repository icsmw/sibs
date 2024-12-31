use crate::*;

impl InferType for Module {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for Module {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let Some(name) = self.get_name() else {
            return Err(LinkedErr::token(E::InvalidModuleName, &self.sig));
        };
        scx.tys.open(&self.uuid);
        scx.fns.enter(name);
        for node in self.nodes.iter() {
            node.initialize(scx)?;
        }
        scx.fns.leave();
        scx.tys
            .close()
            .map_err(|err| LinkedErr::token(err.into(), &self.name))?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Module {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let Some(name) = self.get_name() else {
            return Err(LinkedErr::token(E::InvalidModuleName, &self.sig));
        };
        scx.tys.open(&self.uuid);
        scx.fns.enter(name);
        for node in self.nodes.iter() {
            node.finalize(scx)?;
        }
        scx.fns.leave();
        scx.tys
            .close()
            .map_err(|err| LinkedErr::token(err.into(), &self.name))?;
        Ok(())
    }
}
