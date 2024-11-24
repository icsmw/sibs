use crate::*;

impl InferType for Task {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.block.infer_type(tcx)
    }
}

impl Initialize for Task {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.initialize(tcx))?;
        self.block.initialize(tcx)
    }
}
