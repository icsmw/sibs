use crate::*;

impl InferType for Closure {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        self.block.infer_type(scx)
    }
}

impl Initialize for Closure {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        self.block.initialize(scx)
    }
}

impl Finalization for Closure {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        self.block.finalize(scx)
    }
}
