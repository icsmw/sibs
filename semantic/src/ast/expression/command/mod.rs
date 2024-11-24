use crate::*;

impl InferType for Command {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for CommandPart {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        if let CommandPart::Expression(n) = self {
            n.initialize(tcx)
        } else {
            Ok(())
        }
    }
}

impl Initialize for Command {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(tcx))?;
        Ok(())
    }
}
