use crate::*;

impl InferType for BinaryExp {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let left = self.left.infer_type(tcx)?;
        let right = self.right.infer_type(tcx)?;
        if !left.numeric() {
            Err(LinkedErr::by_link(
                E::ExpectedNumericType(left),
                &self.into(),
            ))
        } else if !right.numeric() {
            Err(LinkedErr::by_link(
                E::ExpectedNumericType(right),
                &self.into(),
            ))
        } else {
            Ok(
                if matches!(left, DataType::F64) | matches!(right, DataType::F64) {
                    DataType::F64
                } else {
                    DataType::Isize
                },
            )
        }
    }
}

impl Initialize for BinaryExp {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.left.initialize(tcx)?;
        self.operator.initialize(tcx)?;
        self.right.initialize(tcx)?;
        self.infer_type(tcx).map(|_| ())
    }
}
