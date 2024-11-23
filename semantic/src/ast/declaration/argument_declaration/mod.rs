use crate::*;

impl InferType for ArgumentDeclaration {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.r#type.infer_type(tcx)
    }
}
