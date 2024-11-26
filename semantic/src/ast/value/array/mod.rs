#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Array {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        let tys = self
            .els
            .iter()
            .map(|n| n.infer_type(tcx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            return Ok(DataType::Vec(Box::new(DataType::Undefined)));
        }
        let first = &tys[0];
        if tys.iter().all(|ty| ty == first) {
            Ok(DataType::Vec(Box::new(first.clone())))
        } else {
            Err(LinkedErr::by_link(
                E::DismatchTypes(
                    tys.iter()
                        .map(|ty| ty.id().to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                ),
                &self.into(),
            ))
        }
    }
}

impl Initialize for Array {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.els.iter().try_for_each(|n| n.initialize(tcx))?;
        self.els
            .iter()
            .try_for_each(|n| n.infer_type(tcx).map(|_| ()))?;
        self.infer_type(tcx).map(|_| ())
    }
}
