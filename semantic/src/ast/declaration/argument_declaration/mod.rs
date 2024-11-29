use crate::*;

impl InferType for ArgumentDeclaration {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.r#type.infer_type(tcx)
    }
}

impl Initialize for ArgumentDeclaration {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.r#type.initialize(tcx)?;
        if let Node::Expression(Expression::Variable(variable)) = self.variable.as_ref() {
            let ty = self.infer_type(tcx)?;
            tcx.insert(&variable.ident, ty)
                .map_err(|e| LinkedErr::by_link(e, &self.into()))?;
            self.variable.initialize(tcx)?;
            Ok(())
        } else {
            Err(LinkedErr::by_link(
                E::UnexpectedNode(self.variable.id()),
                &self.into(),
            ))
        }
    }
}
