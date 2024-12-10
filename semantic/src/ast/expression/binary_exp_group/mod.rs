#[cfg(test)]
mod tests;

use crate::*;

impl InferType for BinaryExpGroup {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let ty = self.node.infer_type(tcx)?;
        if !ty.numeric() {
            Err(LinkedErr::by_node(E::ExpectedNumericType(ty), &self.node))
        } else {
            Ok(ty)
        }
    }
}

impl Initialize for BinaryExpGroup {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.node.initialize(tcx)?;
        self.infer_type(tcx).map(|_| ())
    }
}
