use crate::*;

impl InferType for VariableVariants {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let tys = self
            .variants
            .iter()
            .map(|n| n.infer_type(tcx))
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
                    .map(|ty| ty.id().to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )))
        }
    }
}

impl Initialize for VariableVariants {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.variants.iter().try_for_each(|n| n.initialize(tcx))?;
        Ok(())
    }
}
