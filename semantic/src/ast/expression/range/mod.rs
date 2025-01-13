#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Range {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Range.into())
    }
}

impl Initialize for Range {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.initialize(scx)?;
        self.right.initialize(scx)
    }
}

impl Finalization for Range {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.finalize(scx)?;
        self.right.finalize(scx)?;
        let left = self.left.infer_type(scx)?;
        if !matches!(left, Ty::Determined(DeterminedTy::Num)) {
            return Err(LinkedErr::from(
                E::DismatchTypes(format!("Num and {left}")),
                &self.left,
            ));
        }
        let right = self.right.infer_type(scx)?;
        if !matches!(left, Ty::Determined(DeterminedTy::Num)) {
            return Err(LinkedErr::from(
                E::DismatchTypes(format!("Num and {right}")),
                &self.right,
            ));
        }
        Ok(())
    }
}
