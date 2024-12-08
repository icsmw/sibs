#[cfg(test)]
mod tests;

use crate::*;

impl InferType for FunctionDeclaration {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let mut tcx = TypeContext::default();
        tcx.enter(&self.uuid);
        let ty = self.block.infer_type(&mut tcx)?;
        tcx.leave().map_err(LinkedErr::unlinked)?;
        Ok(ty)
    }
}

impl Initialize for FunctionDeclaration {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        tcx.enter(&self.uuid);
        self.args.iter().try_for_each(|n| n.initialize(tcx))?;
        self.block.initialize(tcx)?;
        tcx.leave().map_err(LinkedErr::unlinked)
    }
}
