use crate::*;

impl InferType for ModuleDeclaration {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for ModuleDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .open(&self.uuid)
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        scx.fns.ufns.enter(&self.name);
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

impl Finalization for ModuleDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .open(&self.uuid)
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        scx.fns.ufns.enter(&self.name);
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
