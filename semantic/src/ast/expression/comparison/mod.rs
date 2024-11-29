use crate::*;

impl InferType for Comparison {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Bool)
    }
}

impl Initialize for Comparison {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.left.initialize(tcx)?;
        self.operator.initialize(tcx)?;
        self.right.initialize(tcx)?;
        let left = self.left.infer_type(tcx)?;
        let right = self.right.infer_type(tcx)?;
        if !left.compatible(&right) {
            Err(LinkedErr::by_link(
                E::DismatchTypes(format!("{left}, {right}")),
                &self.into(),
            ))
        } else {
            Ok(())
        }
    }
}
