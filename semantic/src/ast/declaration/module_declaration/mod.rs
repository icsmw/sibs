use crate::*;

impl InferType for ModuleDeclaration {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for ModuleDeclaration {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.node.initialize(tcx)
    }
}
