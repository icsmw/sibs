// #[cfg(test)]
// mod proptests;
#[cfg(test)]
mod tests;

use crate::*;

impl InferType for VariableDeclaration {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        if let (Some(n_ty), Some(n_assig)) = (self.r#type.as_ref(), self.assignation.as_ref()) {
            let ty = n_ty.infer_type(tcx)?;
            let assig = n_assig.infer_type(tcx)?;
            if ty != assig {
                Err(LinkedErr::between_nodes(
                    E::DismatchTypes(format!("{}, {}", ty.id(), assig.id())),
                    n_ty,
                    n_assig,
                ))
            } else {
                Ok(ty)
            }
        } else if let Some(assig) = self.assignation.as_ref() {
            let ty = assig.infer_type(tcx)?;
            if matches!(ty, DataType::IndeterminateType) {
                Err(LinkedErr::by_node(E::IndeterminateType, assig))
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
                .unwrap_or(DataType::Undefined))
        }
    }
}

impl Initialize for VariableDeclaration {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        if let Some(n) = self.assignation.as_ref() {
            n.initialize(tcx)?;
        }
        if let Some(n) = self.r#type.as_ref() {
            n.initialize(tcx)?;
        }
        if let Node::Declaration(Declaration::VariableName(variable)) = &self.variable.node {
            let ty = self.infer_type(tcx)?;
            tcx.insert(&variable.ident, ty)
                .map_err(|err| LinkedErr::by_node(err, &self.variable))?;
            self.variable.initialize(tcx)?;
            Ok(())
        } else {
            Err(LinkedErr::unlinked(E::UnexpectedNode(
                self.variable.node.id(),
            )))
        }
    }
}
