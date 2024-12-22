use crate::*;

impl InferType for Comparison {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let left = self.left.infer_type(scx)?;
        let right = self.right.infer_type(scx)?;
        if !left.compatible(&right) {
            Err(LinkedErr::between_nodes(
                E::DismatchTypes(format!("{left}, {right}")),
                &self.left,
                &self.right,
            ))
        } else {
            Ok(DataType::Bool)
        }
    }
}

impl Initialize for Comparison {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.initialize(scx)?;
        self.operator.initialize(scx)?;
        self.right.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Comparison {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.finalize(scx)?;
        self.operator.finalize(scx)?;
        self.right.finalize(scx)
    }
}
