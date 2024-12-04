#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Accessor {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let Some(pty) = tcx.get_parent_ty().cloned() else {
            return Err(LinkedErr::by_link(E::AccessorWithoutParent, &self.into()));
        };
        if !matches!(pty, DataType::Vec(..)) {
            return Err(LinkedErr::by_link(
                E::AccessorOnWrongType(pty.to_owned()),
                &self.into(),
            ));
        }
        if let DataType::Vec(inner_ty) = pty {
            let ty = self.node.infer_type(tcx)?;
            if !ty.numeric() {
                return Err(LinkedErr::by_link(
                    E::ExpectedNumericType(ty),
                    &(&self.node).into(),
                ));
            }
            Ok(*inner_ty.to_owned())
        } else {
            Err(LinkedErr::by_link(
                E::AccessorOnWrongType(pty.to_owned()),
                &self.into(),
            ))
        }
    }
}

impl Initialize for Accessor {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.node.initialize(tcx)?;
        self.infer_type(tcx).map(|_| ())
    }
}
