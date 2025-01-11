use crate::*;

impl InferType for Module {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for Module {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let Some(name) = self.get_name() else {
            return Err(LinkedErr::sfrom(E::InvalidModuleName, self));
        };
        scx.tys.open(&self.uuid);
        scx.fns.ufns.enter(name);
        for node in self.nodes.iter() {
            node.initialize(scx)?;
        }
        scx.fns.ufns.leave();
        scx.tys
            .close()
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Module {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let Some(name) = self.get_name() else {
            return Err(LinkedErr::sfrom(E::InvalidModuleName, self));
        };
        scx.tys.open(&self.uuid);
        scx.fns.ufns.enter(name);
        for node in self.nodes.iter() {
            node.finalize(scx)?;
        }
        scx.fns.ufns.leave();
        scx.tys
            .close()
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        Ok(())
    }
}
