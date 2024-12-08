use crate::*;

impl InferType for ArgumentDeclaration {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.r#type.infer_type(tcx)
    }
}

impl Initialize for ArgumentDeclaration {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.r#type.initialize(tcx)?;
        if let Node::Declaration(Declaration::VariableName(variable)) = &self.variable.node {
            let ty = self.infer_type(tcx)?;
            tcx.insert(&variable.ident, ty)
                .map_err(|err| LinkedErr::between_nodes(err, &self.variable, &self.r#type))?;
            self.variable.initialize(tcx)?;
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
