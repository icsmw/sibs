#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Block {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        let mut ty = self
            .nodes
            .last()
            .map(|n| n.infer_type(scx))
            .unwrap_or_else(|| Ok(DeterminedTy::Void.into()))?;
        for ret in self
            .lookup(&[NodeTarget::Statement(&[StatementId::Return])])
            .into_iter()
        {
            if ty != ret.node.infer_type(scx)? {
                ty = Ty::Indeterminate;
                break;
            }
        }
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        Ok(ty)
    }
}

impl Initialize for Block {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        Ok(())
    }
}

impl Finalization for Block {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        self.nodes.iter().try_for_each(|n| n.finalize(scx))?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        Ok(())
    }
}

impl SemanticTokensGetter for Block {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.nodes
            .iter()
            .flat_map(|n| n.get_semantic_tokens(stcx))
            .collect()
    }
}
