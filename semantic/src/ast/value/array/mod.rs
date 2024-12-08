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
        if let Some((n, ty)) = tys.iter().enumerate().find(|(_, ty)| ty != &first) {
            Err(LinkedErr::by_node(
                E::DismatchTypes(format!("{first} and {ty}")),
                &self.els[n],
            ))
        } else {
            Ok(DataType::Vec(Box::new(first.clone())))
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
