#[cfg(test)]
mod tests;

use crate::*;

impl InferType for CompoundAssignments {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let Node::Expression(Expression::CompoundAssignmentsOp(op)) = &self.operator.node else {
            return Err(LinkedErr::from(
                E::UnexpectedNode(self.operator.node.id()),
                &self.operator,
            ));
        };
        let left = self.left.infer_type(scx)?;
        let right = self.right.infer_type(scx)?;
        if !left.compatible(&right) {
            return Err(LinkedErr::from(
                E::DismatchTypes(format!("{left}, {right}")),
                self,
            ));
        }
        if !left.numeric() && !op.is_str_compatible() {
            return Err(LinkedErr::from(E::ExpectedNumericType(left), &self.left));
        }
        if op.is_str_compatible()
            && !matches!(
                left,
                Ty::Determined(DeterminedTy::Num)
                    | Ty::Determined(DeterminedTy::Str)
                    | Ty::Determined(DeterminedTy::PathBuf)
            )
        {
            return Err(LinkedErr::from(
                E::DismatchTypes(format!(
                    "actual: {left}; expected {}, {}, {}",
                    Ty::Determined(DeterminedTy::Num),
                    Ty::Determined(DeterminedTy::Str),
                    Ty::Determined(DeterminedTy::PathBuf)
                )),
                self,
            ));
        }
        Ok(left)
    }
}

impl Initialize for CompoundAssignments {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.initialize(scx)?;
        self.operator.initialize(scx)?;
        self.right.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for CompoundAssignments {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.finalize(scx)?;
        self.operator.finalize(scx)?;
        self.right.finalize(scx)
    }
}
