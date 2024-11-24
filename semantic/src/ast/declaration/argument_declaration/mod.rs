use crate::*;

impl InferType for ArgumentDeclaration {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.r#type.infer_type(tcx)
    }
}

impl Initialize for ArgumentDeclaration {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.r#type.initialize(tcx)?;
        self.variable.initialize(tcx)
    }
}
