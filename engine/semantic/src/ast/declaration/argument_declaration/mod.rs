use crate::*;

impl InferType for ArgumentDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        self.r#type.infer_type(scx)
    }
}

impl Initialize for ArgumentDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.r#type.initialize(scx)?;
        if let Node::Declaration(Declaration::VariableName(variable)) = self.variable.get_node() {
            let ty = self.infer_type(scx)?;
            scx.tys
                .insert(
                    &variable.ident,
                    TypeEntity::new(variable.uuid, Some(ty.clone()), Some(ty)),
                )
                .map_err(|err| LinkedErr::from(err.into(), self))?;
            self.variable.initialize(scx)?;
            Ok(())
        } else {
            Err(LinkedErr::from(
                E::UnexpectedNode(self.variable.get_node().id()),
                self,
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
