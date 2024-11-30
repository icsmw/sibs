use crate::*;

impl InferType for BinaryExpGroup {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        // let ty = self.node.infer_type(tcx)?;
        // if !matches!(ty, DataType::Bool) {
        //     Err(LinkedErr::by_link(
        //         E::ExpectedNumericType(ty),
        //         &(&self.node).into(),
        //     ))
        // } else {
        //     Ok(ty)
        // }
        Ok(DataType::Void)
    }
}

impl Initialize for BinaryExpGroup {
    fn initialize(&self, _tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        // self.node.initialize(tcx)?;
        // self.infer_type(tcx).map(|_| ())
        Ok(())
    }
}
