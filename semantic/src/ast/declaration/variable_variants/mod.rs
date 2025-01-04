use crate::*;

impl InferType for VariableVariants {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let tys = self
            .variants
            .iter()
            .map(|n| n.infer_type(scx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            return Err(LinkedErr::unlinked(E::NoVariantsAreDefined));
        }
        let first = &tys[0];
        if tys.iter().all(|ty| ty == first) {
            Ok(first.clone())
        } else {
            Err(LinkedErr::unlinked(E::DismatchTypes(
                tys.iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )))
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
