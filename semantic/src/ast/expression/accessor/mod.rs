#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Accessor {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let Some(pty) = scx
            .tys
            .get_mut()
            .map_err(|err| LinkedErr::from(err.into(), self))?
            .parent
            .get(&self.uuid)
            .cloned()
        else {
            return Err(LinkedErr::from(E::AccessorWithoutParent, self));
        };
        let dpty = pty.determined().ok_or(LinkedErr::from(
            E::FailInferDeterminedType(pty.clone()),
            self,
        ))?;
        if !matches!(dpty, DeterminedTy::Vec(..)) {
            return Err(LinkedErr::from(
                E::AccessorOnWrongType(pty.to_owned()),
                self,
            ));
        }
        if let DeterminedTy::Vec(Some(inner_ty)) = dpty {
            let ty = self.node.infer_type(scx)?;
            if !ty.numeric() {
                return Err(LinkedErr::from(E::ExpectedNumericType(ty), &self.node));
            }
            Ok((*inner_ty.to_owned()).into())
        } else {
            Err(LinkedErr::from(
                E::AccessorOnWrongType(pty.to_owned()),
                self,
            ))
        }
    }
}

impl Initialize for Accessor {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)
    }
}

impl Finalization for Accessor {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.finalize(scx)
    }
}
