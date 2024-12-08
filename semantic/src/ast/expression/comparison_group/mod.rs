use crate::*;

impl InferType for ComparisonGroup {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let ty = self.node.infer_type(tcx)?;
        if !matches!(ty, DataType::Bool) {
            Err(LinkedErr::by_node(E::ExpectedBoolType(ty), &self.node))
        } else {
            Ok(ty)
        }
    }
}

impl Initialize for ComparisonGroup {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.node.initialize(tcx)?;
        self.infer_type(tcx).map(|_| ())
    }
}
