#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Call {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let Some(name) = self.get_name() else {
            return Err(LinkedErr::by_node(E::NoFnCallNodeFound, &self.node));
        };
        let Some(entity) = scx.fns.lookup(&name) else {
            return Err(LinkedErr::by_node(E::FnNotFound(name), &self.node));
        };
        Ok(entity.result.clone())
    }
}

impl Initialize for Call {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)
    }
}

impl Finalization for Call {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if scx
            .tys
            .get()
            .map_err(|err| LinkedErr::by_node(err, &self.node))?
            .parent
            .get()
            .is_none()
        {
            return Err(LinkedErr::by_node(E::CallWithoutParent, &self.node));
        };
        self.node.finalize(scx)?;
        Ok(())
    }
}
