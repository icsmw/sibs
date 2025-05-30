use crate::*;

impl InferType for VariableVariants {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let tys = self
            .variants
            .iter()
            .map(|n| n.infer_type(scx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            return Err(LinkedErr::from(E::NoVariantsAreDefined, self));
        }
        let first = &tys[0];
        if tys.iter().all(|ty| ty == first) {
            Ok(first.clone())
        } else {
            Err(LinkedErr::from(
                E::DismatchTypes(
                    tys.iter()
                        .map(|ty| ty.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                ),
                self,
            ))
        }
    }
}

impl Initialize for VariableVariants {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.variants.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for VariableVariants {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.variants.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}

impl SemanticTokensGetter for VariableVariants {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.variants
            .iter()
            .flat_map(|n| n.get_semantic_tokens(stcx))
            .collect()
    }
}
