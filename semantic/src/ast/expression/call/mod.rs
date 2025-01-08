#[cfg(test)]
mod tests;

use crate::*;

fn redirect_parent_ty(node: &Call, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
    let Some(ty) = scx
        .tys
        .get()
        .map_err(|err| LinkedErr::by_node(err.into(), &node.node))?
        .parent
        .get(&node.uuid)
        .cloned()
    else {
        return Err(LinkedErr::by_node(E::CallWithoutParent, &node.node));
    };
    scx.tys
        .get_mut()
        .map_err(|err| LinkedErr::by_node(err.into(), &node.node))?
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
