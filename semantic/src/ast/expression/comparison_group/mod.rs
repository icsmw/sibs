#[cfg(test)]
mod tests;

use crate::*;

impl InferType for ComparisonGroup {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let ty = self.node.infer_type(scx)?;
        if !matches!(ty, DataType::Bool) {
            Err(LinkedErr::by_node(E::ExpectedBoolType(ty), &self.node))
        } else {
            Ok(ty)
        }
    }
}

impl Initialize for ComparisonGroup {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}
