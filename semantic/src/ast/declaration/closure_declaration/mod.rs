#[cfg(test)]
mod tests;

use crate::*;

impl InferType for ClosureDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        self.ty.infer_type(scx)
    }
}

impl Initialize for ClosureDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

impl Finalization for ClosureDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
