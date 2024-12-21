#[cfg(test)]
mod tests;

use crate::*;

impl InferType for FunctionDeclaration {
    fn infer_type(&self, _tcx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let mut scx = SemanticCx::default();
        scx.tys.enter(&self.uuid);
        let ty = self.block.infer_type(&mut scx)?;
        scx.tys.leave().map_err(LinkedErr::unlinked)?;
        Ok(ty)
    }
}

impl Initialize for FunctionDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys.enter(&self.uuid);
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        self.block.initialize(scx)?;
        scx.tys.leave().map_err(LinkedErr::unlinked)
    }
}
