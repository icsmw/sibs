use crate::*;


impl InferType for VariableVariants {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let tys = self
            .variants
            .iter()
            .map(|n| n.infer_type(tcx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            return Err(LinkedErr::by_link(E::NoVariantsAreDefined, &self.into()));
        }
        let first = &tys[0];
        if tys.iter().all(|ty| ty == first) {
            Ok(first.clone())
        } else {
            Err(LinkedErr::by_link(E::NoVariantsAreDefined, &self.into()))
        }
    }
}
