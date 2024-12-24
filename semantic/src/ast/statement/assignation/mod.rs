use crate::*;

impl InferType for Assignation {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let variable = if let Node::Expression(Expression::Variable(variable)) = &self.left.node {
            variable.ident.to_owned()
        } else {
            return Err(LinkedErr::between_nodes(
                E::UnexpectedNode(self.left.node.id()),
                &self.left,
                &self.right,
            ));
        };
        let left = scx
            .tys
            .lookup(&variable)
            .map_err(|err| LinkedErr::by_node(err, &self.left))?
            .cloned()
            .ok_or(LinkedErr::by_node(
                E::VariableIsNotDefined(variable.clone()),
                &self.left,
            ))?;
        let right = self.right.infer_type(scx)?;
        if matches!(right, DataType::IndeterminateType) {
            return Err(LinkedErr::by_node(E::IndeterminateType, &self.right));
        }
        let Some(annot) = left.annotated.as_ref() else {
            return Err(LinkedErr::by_node(E::IndeterminateType, &self.left));
        };
        if annot.reassignable(&right) {
            scx.tys
                .insert(
                    variable,
                    TypeEntity::new(Some(right), Some(annot.to_owned())),
                )
                .map_err(|e| LinkedErr::between_nodes(e, &self.left, &self.right))?;
            Ok(DataType::Void)
        } else {
            Err(LinkedErr::between_nodes(
                E::DismatchTypes(format!("{annot}, {right}")),
                &self.left,
                &self.right,
            ))
        }
    }
}

impl Initialize for Assignation {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.initialize(scx)?;
        self.right.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Assignation {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.finalize(scx)?;
        self.right.finalize(scx)
    }
}
