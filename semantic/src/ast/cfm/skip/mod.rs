use crate::*;

impl InferType for Skip {
    fn infer_type(&self, _tcx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Void)
    }
}

impl Initialize for SkipTaskArgument {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Self::Value(n) => n.initialize(scx),
            Self::Any => Ok(()),
        }
    }
}

impl Initialize for Skip {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        self.func.initialize(scx)
    }
}
