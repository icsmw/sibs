#[cfg(test)]
mod tests;

use crate::*;

impl InferType for CompoundAssignments {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let left = self.left.infer_type(tcx)?;
        let right = self.right.infer_type(tcx)?;
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
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.left.initialize(tcx)?;
        self.operator.initialize(tcx)?;
        self.right.initialize(tcx)?;
        self.infer_type(tcx).map(|_| ())
    }
}
