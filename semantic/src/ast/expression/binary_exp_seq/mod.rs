use crate::*;

impl InferType for BinaryExpSeq {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let mut out = DataType::Isize;
        for node in self.nodes.iter() {
            let ty = node.infer_type(tcx)?;
            if !ty.numeric() {
                return Err(LinkedErr::by_link(E::ExpectedNumericType(ty), &node.into()));
            }
            if matches!(ty, DataType::F64) {
                out = ty;
            }
        }
        Ok(out)
    }
}

impl Initialize for BinaryExpSeq {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(tcx))?;
        self.infer_type(tcx).map(|_| ())
    }
}
