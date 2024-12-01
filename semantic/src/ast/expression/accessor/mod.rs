#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Accessor {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let ty = self.node.infer_type(tcx)?;
        if !ty.numeric() {
            return Err(LinkedErr::by_link(
                E::ExpectedNumericType(ty),
                &(&self.node).into(),
            ));
        }
        Ok(DataType::Void)
    }
}

impl Initialize for Accessor {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.node.initialize(tcx)?;
        self.infer_type(tcx).map(|_| ())
    }
}
