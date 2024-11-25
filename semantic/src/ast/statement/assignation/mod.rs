use crate::*;

impl InferType for Assignation {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let variable = if let Node::Expression(Expression::Variable(variable)) = self.left.as_ref()
        {
            variable.ident.to_owned()
        } else {
            return Err(LinkedErr::by_link(
                E::UnexpectedNode(self.left.id()),
                &self.into(),
            ));
        };
        let left = tcx
            .lookup(&variable)
            .ok_or(LinkedErr::by_link(
                E::VariableIsNotDefined,
                &(&self.left).into(),
            ))?
            .clone();
        let right = self.right.infer_type(tcx)?;
        if matches!(left, DataType::Undefined) {
            tcx.insert(variable, right)
                .map_err(|e| LinkedErr::by_link(e, &self.into()))?;
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
