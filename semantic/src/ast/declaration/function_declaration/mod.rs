use crate::*;


impl InferType for FunctionDeclaration {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.block.infer_type(tcx)
    }
}
