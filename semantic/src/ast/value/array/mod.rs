#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Array {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let tys = self
            .els
            .iter()
            .map(|n| n.infer_type(scx))
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
