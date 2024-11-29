#[cfg(test)]
mod tests;

use crate::*;

impl InferType for IfCase {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            IfCase::If(_, blk, ..) => blk.infer_type(tcx),
            IfCase::Else(blk, ..) => blk.infer_type(tcx),
        }
    }
}

impl InferType for If {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        if !self.cases.iter().any(|c| matches!(c, IfCase::Else(..))) {
            return Ok(DataType::IndeterminateType);
        }
        let tys = self
            .cases
            .iter()
            .map(|n| n.infer_type(tcx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            return Err(LinkedErr::by_link(E::InvalidIfStatement, &self.into()));
        };
        let first = &tys[0];
        if tys.iter().all(|ty| first.compatible(ty)) {
            Ok(first.clone())
        } else {
            Ok(DataType::IndeterminateType)
        }
    }
}

impl Initialize for IfCase {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        match self {
            IfCase::If(con, blk, ..) => {
                con.initialize(tcx)?;
                blk.initialize(tcx)
            }
            IfCase::Else(blk, ..) => blk.initialize(tcx),
        }
    }
}

impl Initialize for If {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.cases.iter().try_for_each(|n| n.initialize(tcx))?;
        self.infer_type(tcx).map(|_| ())
    }
}
