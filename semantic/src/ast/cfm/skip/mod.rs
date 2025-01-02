use crate::*;

impl InferType for Skip {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
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

impl Finalization for SkipTaskArgument {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Self::Value(n) => n.finalize(scx),
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

impl Finalization for Skip {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        self.func.finalize(scx)
    }
}
