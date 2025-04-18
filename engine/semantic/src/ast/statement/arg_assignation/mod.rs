use crate::*;

impl InferType for ArgumentAssignation {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let variable = if let Node::Expression(Expression::Variable(variable)) = &self.left.node {
            variable.ident.to_owned()
        } else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.left.node.id()),
                &self.left,
            ));
        };
        let left = scx
            .tys
            .lookup(&variable)
            .map_err(|err| LinkedErr::from(err.into(), &self.left))?
            .cloned()
            .ok_or(LinkedErr::from(
                E::VariableIsNotDefined(variable.clone()),
                &self.left,
            ))?;
        let right = self.right.infer_type(scx)?;
        if matches!(right, Ty::Indeterminate) {
            return Err(LinkedErr::from(E::IndeterminateType, &self.right));
        }
        let Some(annot) = left.annotated.as_ref() else {
            return Err(LinkedErr::from(E::IndeterminateType, &self.left));
        };
        if annot.reassignable(&right) {
            scx.tys
                .insert(
                    variable,
                    TypeEntity::new(Some(right), Some(annot.to_owned())),
                )
                .map_err(|err| LinkedErr::from(err.into(), self))?;
            Ok(DeterminedTy::Void.into())
        } else {
            Err(LinkedErr::from(
                E::DismatchTypes(format!("{annot}, {right}")),
                self,
            ))
        }
    }
}

impl Initialize for ArgumentAssignation {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.initialize(scx)?;
        self.right.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for ArgumentAssignation {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.finalize(scx)?;
        self.right.finalize(scx)
    }
}
