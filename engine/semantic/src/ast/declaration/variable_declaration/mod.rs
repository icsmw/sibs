// #[cfg(test)]
// mod proptests;
#[cfg(test)]
mod tests;

use crate::*;

impl InferType for VariableDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let Node::Declaration(Declaration::VariableName(variable)) = self.variable.get_node()
        else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.variable.get_node().id()),
                &self.variable,
            ));
        };
        let Some(ty) = scx
            .tys
            .lookup(&variable.ident)
            .map_err(|err| LinkedErr::from(err.into(), &self.variable))?
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
        let Node::Declaration(Declaration::VariableName(variable)) = self.variable.get_node()
        else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.variable.get_node().id()),
                &self.variable,
            ));
        };
        if let (Some(n_ty), Some(n_assig)) = (self.r#type.as_ref(), self.assignation.as_ref()) {
            let annot = n_ty.infer_type(scx)?;
            let assig = n_assig.infer_type(scx)?;
            if annot != assig {
                return Err(LinkedErr::from(
                    E::DismatchTypes(format!("{}, {}", annot, assig)),
                    n_ty,
                ));
            }
            scx.tys
                .insert(
                    &variable.ident,
                    TypeEntity::new(*&self.uuid, self.get_position(), Some(assig), Some(annot)),
                )
                .map_err(|err| LinkedErr::from(err.into(), &self.variable))?;
        } else if let Some(node) = self.assignation.as_ref() {
            let assig = node.infer_type(scx)?;
            if matches!(assig, Ty::Indeterminate) {
                return Err(LinkedErr::from(E::IndeterminateType, node));
            }
            scx.tys
                .insert(
                    &variable.ident,
                    TypeEntity::new(
                        *&self.uuid,
                        self.get_position(),
                        Some(assig.clone()),
                        Some(assig),
                    ),
                )
                .map_err(|err| LinkedErr::from(err.into(), &self.variable))?;
        } else if let Some(node) = self.r#type.as_ref() {
            let annot = node.infer_type(scx)?;
            if matches!(annot, Ty::Indeterminate) {
                return Err(LinkedErr::from(E::IndeterminateType, node));
            }
            scx.tys
                .insert(
                    &variable.ident,
                    TypeEntity::new(*&self.uuid, self.get_position(), None, Some(annot)),
                )
                .map_err(|err| LinkedErr::from(err.into(), &self.variable))?;
        } else {
            scx.tys
                .insert(
                    &variable.ident,
                    TypeEntity::new(*&self.uuid, self.get_position(), None, Some(Ty::Undefined)),
                )
                .map_err(|err| LinkedErr::from(err.into(), &self.variable))?;
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

impl SemanticTokensGetter for VariableDeclaration {
    fn get_semantic_tokens(&self, _stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![LinkedSemanticToken::from_token(
            &self.token,
            SemanticToken::Keyword,
        )];
        tokens.extend(
            self.variable
                .get_semantic_tokens(SemanticTokenContext::VariableDeclaration),
        );
        self.r#type.as_ref().map(|n| {
            tokens.extend(n.get_semantic_tokens(SemanticTokenContext::VariableDeclaration))
        });
        self.assignation.as_ref().map(|n| {
            tokens.extend(n.get_semantic_tokens(SemanticTokenContext::VariableDeclaration))
        });
        tokens
    }
}
