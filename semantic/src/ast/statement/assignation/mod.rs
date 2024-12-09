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
            .cloned()
            .ok_or(LinkedErr::by_node(E::VariableIsNotDefined, &self.left))?;
        let right = self.right.infer_type(tcx)?;
        if matches!(right, DataType::IndeterminateType) {
            return Err(LinkedErr::by_node(E::IndeterminateType, &self.right));
        }
        let Some(annot) = left.annotated.as_ref() else {
            return Err(LinkedErr::by_node(E::IndeterminateType, &self.left));
        };
        if annot.reassignable(&right) {
            tcx.insert(
                variable,
                EntityType::new(Some(right), Some(annot.to_owned())),
            )
            .map_err(|e| LinkedErr::between_nodes(e, &self.left, &self.right))?;
            Ok(DataType::Void)
        } else {
            println!(">>>>>>>>>>>>>>>>>> 00000");
            Err(LinkedErr::between_nodes(
                E::DismatchTypes(format!("{annot}, {right}")),
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
