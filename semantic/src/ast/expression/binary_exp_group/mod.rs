#[cfg(test)]
mod tests;

use crate::*;

impl InferType for BinaryExpGroup {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let ty = self.node.infer_type(scx)?;
        if !ty.numeric() {
            Err(LinkedErr::by_node(E::ExpectedNumericType(ty), &self.node))
        } else {
            Ok(ty)
        }
    }
}

impl Initialize for BinaryExpGroup {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}
