use crate::*;

impl InferType for InterpolatedString {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminatedTy::Str.into())
    }
}

impl Initialize for InterpolatedStringPart {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let InterpolatedStringPart::Expression(n) = self {
            n.initialize(scx)
        } else {
            Ok(())
        }
    }
}

impl Finalization for InterpolatedStringPart {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let InterpolatedStringPart::Expression(n) = self {
            n.finalize(scx)
        } else {
            Ok(())
        }
    }
}

impl Initialize for InterpolatedString {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for InterpolatedString {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}
