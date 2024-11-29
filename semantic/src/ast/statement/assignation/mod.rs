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
        if matches!(right, DataType::IndeterminateType) {
            return Err(LinkedErr::by_link(
                E::IndeterminateType,
                &(&self.left).into(),
            ));
        }
        if left.reassignable(&right) {
            tcx.insert(variable, right)
                .map_err(|e| LinkedErr::by_link(e, &self.into()))?;
            Ok(DataType::Void)
        } else {
            Err(LinkedErr::by_link(
                E::DismatchTypes(format!("{left}, {right}")),
                &self.into(),
            ))
        }
    }
}

impl Initialize for Assignation {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.left.initialize(tcx)?;
        self.right.initialize(tcx)?;
        self.infer_type(tcx).map(|_| ())
    }
}
