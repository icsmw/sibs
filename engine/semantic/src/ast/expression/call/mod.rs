#[cfg(test)]
mod tests;

use crate::*;

fn redirect_parent_ty(node: &Call, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
    let Some(ty) = scx
        .tys
        .get()
        .map_err(|err| LinkedErr::from(err.into(), &node.node))?
        .parent
        .get(&node.uuid)
        .cloned()
    else {
        return Err(LinkedErr::from(E::CallWithoutParent, &node.node));
    };
    scx.tys
        .get_mut()
        .map_err(|err| LinkedErr::from(err.into(), &node.node))?
        .parent
        .set(node.node.uuid(), ty);
    Ok(())
}

impl InferType for Call {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        redirect_parent_ty(self, scx)?;
        self.node.infer_type(scx)
    }
}

impl Initialize for Call {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)
    }
}

impl Finalization for Call {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        redirect_parent_ty(self, scx)?;
        self.node.finalize(scx)
    }
}

impl SemanticTokensGetter for Call {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.node.get_semantic_tokens(stcx)
    }
}
