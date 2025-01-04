// #[cfg(test)]
// mod proptests;
#[cfg(test)]
mod tests;

use crate::*;

impl InferType for VariableDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let Node::Declaration(Declaration::VariableName(variable)) = &self.variable.node else {
            return Err(LinkedErr::by_node(
                E::UnexpectedNode(self.variable.node.id()),
                &self.variable,
            ));
        };
        let Some(ty) = scx
            .tys
            .lookup(&variable.ident)
            .map_err(|err| LinkedErr::by_node(err.into(), &self.variable))?
        else {
            return Ok(Ty::Indeterminate);
        };
        Ok(ty.assigned.clone().unwrap_or(Ty::Indeterminate))
    }
}

impl Initialize for VariableDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let Some(n) = self.assignation.as_ref() {
            n.initialize(scx)?;
        }
        if let Some(n) = self.r#type.as_ref() {
            n.initialize(scx)?;
        }
        self.variable.initialize(scx)?;
        let Node::Declaration(Declaration::VariableName(variable)) = &self.variable.node else {
            return Err(LinkedErr::by_node(
                E::UnexpectedNode(self.variable.node.id()),
                &self.variable,
            ));
        };
        if let (Some(n_ty), Some(n_assig)) = (self.r#type.as_ref(), self.assignation.as_ref()) {
            let annot = n_ty.infer_type(scx)?;
            let assig = n_assig.infer_type(scx)?;
            if annot != assig {
                return Err(LinkedErr::between_nodes(
                    E::DismatchTypes(format!("{}, {}", annot, assig)),
                    n_ty,
                    n_assig,
                ));
            }
            scx.tys
                .insert(&variable.ident, TypeEntity::new(Some(assig), Some(annot)))
                .map_err(|err| LinkedErr::by_node(err.into(), &self.variable))?;
        } else if let Some(node) = self.assignation.as_ref() {
            let assig = node.infer_type(scx)?;
            if matches!(assig, Ty::Indeterminate) {
                return Err(LinkedErr::by_node(E::IndeterminateType, node));
            }
            scx.tys
                .insert(
                    &variable.ident,
                    TypeEntity::new(Some(assig.clone()), Some(assig)),
                )
                .map_err(|err| LinkedErr::by_node(err.into(), &self.variable))?;
        } else if let Some(node) = self.r#type.as_ref() {
            let annot = node.infer_type(scx)?;
            if matches!(annot, Ty::Indeterminate) {
                return Err(LinkedErr::by_node(E::IndeterminateType, node));
            }
            scx.tys
                .insert(&variable.ident, TypeEntity::new(None, Some(annot)))
                .map_err(|err| LinkedErr::by_node(err.into(), &self.variable))?;
        } else {
            scx.tys
                .insert(&variable.ident, TypeEntity::new(None, Some(Ty::Undefined)))
                .map_err(|err| LinkedErr::by_node(err.into(), &self.variable))?;
        }
        Ok(())
    }
}

impl Finalization for VariableDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let Some(n) = self.r#type.as_ref() {
            n.finalize(scx)?;
        }
        if let Some(n) = self.assignation.as_ref() {
            n.finalize(scx)?;
        }
        self.variable.finalize(scx)
    }
}
