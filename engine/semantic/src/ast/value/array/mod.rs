#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Array {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let tys = self
            .els
            .iter()
            .map(|n| n.infer_type(scx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            return Ok(DeterminedTy::Vec(None).into());
        }
        let first = tys[0].determined().cloned().ok_or(LinkedErr::from(
            E::FailInferDeterminedType(tys[0].clone()),
            &self.els[0],
        ))?;
        if let Some((n, ty)) = tys.iter().enumerate().find(|(_, ty)| !ty.equal(&first)) {
            Err(LinkedErr::from(
                E::DismatchTypes(format!("{first} and {ty}")),
                &self.els[n],
            ))
        } else {
            Ok(DeterminedTy::Vec(Some(Box::new(first))).into())
        }
    }
}

impl Initialize for Array {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.els.iter().try_for_each(|n| n.initialize(scx))?;
        self.els
            .iter()
            .try_for_each(|n| n.infer_type(scx).map(|_| ()))?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Array {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.els.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}

impl SemanticTokensGetter for Array {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.els
            .iter()
            .flat_map(|n| n.get_semantic_tokens(stcx))
            .collect()
    }
}
