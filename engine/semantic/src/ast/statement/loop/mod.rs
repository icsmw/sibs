#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Loop {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for Loop {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
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
        if nodes.is_empty() {
            return Err(LinkedErr::from(E::NotBreakableLoop, self));
        }
        for found in nodes.iter() {
            match found.node.get_node() {
                Node::Statement(Statement::Break(node)) => {
                    if !node.is_assigned() {
                        return Err(LinkedErr::from(E::NotAssignedBreak, node));
                    }
                }
                Node::Statement(Statement::Return(node)) => {
                    if !node.is_assigned() {
                        return Err(LinkedErr::from(E::NotAssignedReturn, node));
                    }
                }
                _ => {
                    return Err(LinkedErr::from(E::NotBreakableLoop, self));
                }
            }
        }
        if !nodes.iter().any(|found| {
            if let Node::Statement(Statement::Break(node)) = found.node.get_node() {
                node.is_target(&self.uuid)
            } else if let Node::Statement(Statement::Return(node)) = found.node.get_node() {
                node.is_target_included(&self.uuid)
            } else {
                false
            }
        }) {
            return Err(LinkedErr::from(E::NotAssignedBreak, self));
        };
        self.block.initialize(scx)?;
        Ok(())
    }
}

impl Finalization for Loop {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.block.finalize(scx)
    }
}
