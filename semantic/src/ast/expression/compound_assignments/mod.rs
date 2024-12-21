#[cfg(test)]
mod tests;

use crate::*;

impl InferType for CompoundAssignments {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let left = self.left.infer_type(scx)?;
        let right = self.right.infer_type(scx)?;
        if !left.numeric() {
            Err(LinkedErr::by_node(E::ExpectedNumericType(left), &self.left))
        } else if !right.numeric() {
            Err(LinkedErr::by_node(
                E::ExpectedNumericType(right),
                &self.right,
            ))
        } else if !left.compatible(&right) {
            Err(LinkedErr::between_nodes(
                E::DismatchTypes(format!("{left}, {right}")),
                &self.left,
                &self.right,
            ))
        } else {
            Ok(DataType::Void)
        }
    }
}

impl Initialize for CompoundAssignments {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.initialize(scx)?;
        self.operator.initialize(scx)?;
        self.right.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}
