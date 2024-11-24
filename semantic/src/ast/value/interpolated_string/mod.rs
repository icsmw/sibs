use crate::*;

impl InferType for InterpolatedString {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::String)
    }
}

impl Initialize for InterpolatedStringPart {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        if let InterpolatedStringPart::Expression(n) = self {
            n.initialize(tcx)
        } else {
            Ok(())
        }
    }
}

impl Initialize for InterpolatedString {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(tcx))?;
        Ok(())
    }
}
