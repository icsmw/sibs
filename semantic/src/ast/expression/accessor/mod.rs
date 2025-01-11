#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Accessor {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let Some(pty) = scx
            .tys
            .get_mut()
            .map_err(|err| LinkedErr::between(err.into(), &self.open, &self.close))?
            .parent
            .get(&self.uuid)
            .cloned()
        else {
            return Err(LinkedErr::between(
                E::AccessorWithoutParent,
                &self.open,
                &self.close,
            ));
        };
        let dpty = pty.determined().ok_or(LinkedErr::between(
            E::FailInferDeterminedType(pty.clone()),
            &self.open,
            &self.close,
        ))?;
        if !matches!(dpty, DeterminedTy::Vec(..)) {
            return Err(LinkedErr::between(
                E::AccessorOnWrongType(pty.to_owned()),
                &self.open,
                &self.close,
            ));
        }
        if let DeterminedTy::Vec(Some(inner_ty)) = dpty {
            let ty = self.node.infer_type(scx)?;
            if !ty.numeric() {
                return Err(LinkedErr::from(E::ExpectedNumericType(ty), &self.node));
            }
            Ok((*inner_ty.to_owned()).into())
        } else {
            Err(LinkedErr::between(
                E::AccessorOnWrongType(pty.to_owned()),
                &self.open,
                &self.close,
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
