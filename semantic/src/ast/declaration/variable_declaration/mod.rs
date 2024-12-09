// #[cfg(test)]
// mod proptests;
#[cfg(test)]
mod tests;

use crate::*;

impl InferType for VariableDeclaration {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let Node::Declaration(Declaration::VariableName(variable)) = &self.variable.node else {
            return Err(LinkedErr::by_node(
                E::UnexpectedNode(self.variable.node.id()),
                &self.variable,
            ));
        };
        let Some(ty) = tcx.lookup(&variable.ident) else {
            return Ok(DataType::IndeterminateType);
        };
        Ok(ty
            .assigned
            .clone()
            .unwrap_or_else(|| DataType::IndeterminateType))
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
        self.variable.initialize(tcx)?;
        let Node::Declaration(Declaration::VariableName(variable)) = &self.variable.node else {
            return Err(LinkedErr::by_node(
                E::UnexpectedNode(self.variable.node.id()),
                &self.variable,
            ));
        };
        if let (Some(n_ty), Some(n_assig)) = (self.r#type.as_ref(), self.assignation.as_ref()) {
            let annot = n_ty.infer_type(tcx)?;
            let assig = n_assig.infer_type(tcx)?;
            if annot != assig {
                return Err(LinkedErr::between_nodes(
                    E::DismatchTypes(format!("{}, {}", annot.id(), assig.id())),
                    n_ty,
                    n_assig,
                ));
            }
            tcx.insert(&variable.ident, EntityType::new(Some(assig), Some(annot)))
                .map_err(|err| LinkedErr::by_node(err, &self.variable))?;
        } else if let Some(node) = self.assignation.as_ref() {
            let assig = node.infer_type(tcx)?;
            if matches!(assig, DataType::IndeterminateType) {
                return Err(LinkedErr::by_node(E::IndeterminateType, node));
            }
            tcx.insert(
                &variable.ident,
                EntityType::new(Some(assig.clone()), Some(assig)),
            )
            .map_err(|err| LinkedErr::by_node(err, &self.variable))?;
        } else if let Some(node) = self.r#type.as_ref() {
            let annot = node.infer_type(tcx)?;
            if matches!(annot, DataType::IndeterminateType) {
                return Err(LinkedErr::by_node(E::IndeterminateType, node));
            }
            tcx.insert(&variable.ident, EntityType::new(None, Some(annot)))
                .map_err(|err| LinkedErr::by_node(err, &self.variable))?;
        } else {
            tcx.insert(
                &variable.ident,
                EntityType::new(None, Some(DataType::Undefined)),
            )
            .map_err(|err| LinkedErr::by_node(err, &self.variable))?;
        }
        Ok(())
    }
}
