use crate::*;

impl InferType for ArgumentDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        self.r#type.infer_type(scx)
    }
}

impl Initialize for ArgumentDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.r#type.initialize(scx)?;
        if let Node::Declaration(Declaration::VariableName(variable)) = &self.variable.node {
            let ty = self.infer_type(scx)?;
            scx.tys
                .insert(&variable.ident, TypeEntity::new(Some(ty.clone()), Some(ty)))
                .map_err(|err| {
                    LinkedErr::between_nodes(err.into(), &self.variable, &self.r#type)
                })?;
            self.variable.initialize(scx)?;
            Ok(())
        } else {
            Err(LinkedErr::between_nodes(
                E::UnexpectedNode(self.variable.node.id()),
                &self.variable,
                &self.r#type,
            ))
        }
    }
}

impl Finalization for ArgumentDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.r#type.finalize(scx)?;
        self.variable.finalize(scx)
    }
}
