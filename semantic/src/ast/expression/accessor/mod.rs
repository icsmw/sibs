#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Accessor {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let Some(pty) = scx.tys.parent.get().cloned() else {
            return Err(LinkedErr::between(
                E::AccessorWithoutParent,
                &self.open,
                &self.close,
            ));
        };
        if !matches!(pty, DataType::Vec(..)) {
            return Err(LinkedErr::between(
                E::AccessorOnWrongType(pty.to_owned()),
                &self.open,
                &self.close,
            ));
        }
        if let DataType::Vec(inner_ty) = pty {
            let ty = self.node.infer_type(scx)?;
            if !ty.numeric() {
                return Err(LinkedErr::by_node(E::ExpectedNumericType(ty), &self.node));
            }
            Ok(*inner_ty.to_owned())
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
        self.node.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Accessor {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.finalize(scx)
    }
}
