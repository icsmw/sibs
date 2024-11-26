use crate::*;

impl InferType for Return {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.node
            .as_ref()
            .map(|n| n.infer_type(tcx))
            .unwrap_or_else(|| Ok(DataType::Void))
    }
}

impl Initialize for Return {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        if let Some(n) = self.node.as_ref() {
            n.initialize(tcx)?;
            n.infer_type(tcx).map(|_| ())?;
        }
        self.infer_type(tcx).map(|_| ())
    }
}
