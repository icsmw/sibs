use crate::*;

impl InferType for Skip {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for SkipTaskArgument {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        match self {
            Self::Value(n) => n.initialize(tcx),
            Self::Any => Ok(()),
        }
    }
}

impl Initialize for Skip {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.initialize(tcx))?;
        self.func.initialize(tcx)
    }
}
