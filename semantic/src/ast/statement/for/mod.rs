#[cfg(test)]
mod tests;

use crate::*;

impl InferType for For {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for For {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        self.elements.initialize(scx)?;
        let ty = match self.elements.infer_type(scx)? {
            Ty::Determined(DeterminedTy::Vec(None)) => {
                return Err(LinkedErr::from(E::IndeterminateType, &self.elements))
            }
            Ty::Determined(DeterminedTy::Vec(Some(ty))) => Ty::Determined(*ty),
            Ty::Determined(DeterminedTy::Range) => Ty::Determined(DeterminedTy::Num),
            Ty::Determined(DeterminedTy::Str) => Ty::Determined(DeterminedTy::Str),
            _ => return Err(LinkedErr::from(E::InvalidIterationSource, &self.elements)),
        };
        let el = if let Node::Expression(Expression::Variable(el)) = &self.element.node {
            el.ident.to_owned()
        } else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.element.node.id()),
                &self.element,
            ));
        };
        scx.tys
            .insert(el, TypeEntity::new(Some(ty.clone()), Some(ty)))
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        if let Some(index) = self.index.as_ref() {
            let el = if let Node::Expression(Expression::Variable(el)) = &index.node {
                el.ident.to_owned()
            } else {
                return Err(LinkedErr::from(
                    E::UnexpectedNode(index.node.id()),
                    &index.node,
                ));
            };
            scx.tys
                .insert(
                    el,
                    TypeEntity::new(
                        Some(Ty::Determined(DeterminedTy::Num)),
                        Some(Ty::Determined(DeterminedTy::Num)),
                    ),
                )
                .map_err(|err| LinkedErr::from(err.into(), self))?;
        };
        self.block.initialize(scx)?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        if let Some(node) = self
            .block
            .lookup(&[NodeTarget::Declaration(&[
                DeclarationId::FunctionDeclaration,
                DeclarationId::ClosureDeclaration,
            ])])
            .first()
        {
            return Err(LinkedErr::from(E::NotAllowedFnDeclaration, node.node));
        }
        let nodes = self.block.lookup(&[NodeTarget::Statement(&[
            StatementId::Break,
            StatementId::Return,
        ])]);
        for found in nodes.into_iter() {
            match &found.node.node {
                Node::Statement(Statement::Break(node)) => {
                    if node.target.is_none() {
                        return Err(LinkedErr::from(E::NotAssignedBreak, node));
                    }
                }
                Node::Statement(Statement::Return(node)) => {
                    if !node.is_assigned() {
                        return Err(LinkedErr::from(E::NotAssignedReturn, node));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl Finalization for For {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        self.elements.finalize(scx)?;
        self.block.finalize(scx)?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        Ok(())
    }
}
