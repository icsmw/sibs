use crate::*;

impl InferType for Assignation {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let left = tcx
            .get_annotation(self.left.uuid())
            .ok_or(LinkedErr::by_link(
                E::VariableIsNotDefined,
                &(&self.left).into(),
            ))?
            .clone();
        let right = self.right.infer_type(tcx)?;
        if matches!(left, DataType::Undefined) {
            tcx.annotate(self.left.uuid(), right);
        } else if !left.compatible(&right) {
            return Err(LinkedErr::by_link(
                E::DismatchTypes(format!("{left}, {right}")),
                &self.into(),
            ));
        }
        Ok(DataType::Void)
    }
}

impl Initialize for Assignation {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.left.initialize(tcx)?;
        self.right.initialize(tcx)
    }
}
