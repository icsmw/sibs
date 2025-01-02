#[cfg(test)]
mod tests;

use crate::*;

impl InferType for BinaryExp {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let left = self.left.infer_type(scx)?;
        let right = self.right.infer_type(scx)?;
        if !left.numeric() {
            Err(LinkedErr::between_nodes(
                E::ExpectedNumericType(left),
                &self.left,
                &self.right,
            ))
        } else if !right.numeric() {
            Err(LinkedErr::between_nodes(
                E::ExpectedNumericType(right),
                &self.left,
                &self.right,
            ))
        } else {
            Ok(DeterminedTy::Num.into())
        }
    }
}

impl Initialize for BinaryExp {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.initialize(scx)?;
        self.operator.initialize(scx)?;
        self.right.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for BinaryExp {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.finalize(scx)?;
        self.operator.finalize(scx)?;
        self.right.finalize(scx)
    }
}
