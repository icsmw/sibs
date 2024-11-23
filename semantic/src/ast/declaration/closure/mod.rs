use crate::*;


impl InferType for Closure {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.block.infer_type(tcx)
    }
}
