#[cfg(test)]
mod tests;

use crate::*;

impl InferType for BinaryExp {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
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
            Ok(DataType::Num)
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
