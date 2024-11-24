use crate::*;

impl InferType for VariableDeclaration {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        if let (Some(ty), Some(assig)) = (self.r#type.as_ref(), self.assignation.as_ref()) {
            let ty = ty.infer_type(tcx)?;
            let assig = assig.infer_type(tcx)?;
            if ty != assig {
                Err(LinkedErr::by_link(
                    E::DismatchTypes(format!("{}, {}", ty.id(), assig.id())),
                    &self.into(),
                ))
            } else {
                Ok(ty)
            }
        } else {
            Ok(self
                .r#type
                .as_ref()
                .map(|ty| ty.infer_type(tcx))
                .or_else(|| self.assignation.as_ref().map(|ty| ty.infer_type(tcx)))
                .transpose()?
                .unwrap_or_else(|| DataType::Undefined))
        }
    }
}

impl Initialize for VariableDeclaration {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.variable.initialize(tcx)?;
        if let Some(n) = self.assignation.as_ref() {
            n.initialize(tcx)?;
        }
        if let Some(n) = self.r#type.as_ref() {
            n.initialize(tcx)?;
        }
        Ok(())
    }
}
