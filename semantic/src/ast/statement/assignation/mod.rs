use crate::*;

impl InferType for Assignation {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let variable = if let Node::Expression(Expression::Variable(variable)) = &self.left.node {
            variable.ident.to_owned()
        } else {
            return Err(LinkedErr::between_nodes(
                E::UnexpectedNode(self.left.node.id()),
                &self.left,
                &self.right,
            ));
        };
        let left = tcx
            .lookup(&variable)
            .ok_or(LinkedErr::by_node(E::VariableIsNotDefined, &self.left))?
            .clone();
        let right = self.right.infer_type(tcx)?;
        if matches!(right, DataType::IndeterminateType) {
            return Err(LinkedErr::by_node(E::IndeterminateType, &self.left));
        }
        if left.reassignable(&right) {
            tcx.insert(variable, right)
                .map_err(|e| LinkedErr::between_nodes(e, &self.left, &self.right))?;
            Ok(DataType::Void)
        } else {
            Err(LinkedErr::between_nodes(
                E::DismatchTypes(format!("{left}, {right}")),
                &self.left,
                &self.right,
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
