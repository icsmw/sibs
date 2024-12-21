#[cfg(test)]
mod tests;

use crate::*;

impl InferType for IfCase {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        match self {
            IfCase::If(_, blk, ..) => blk.infer_type(scx),
            IfCase::Else(blk, ..) => blk.infer_type(scx),
        }
    }
}

impl InferType for If {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        if !self.cases.iter().any(|c| matches!(c, IfCase::Else(..))) {
            return Ok(DataType::IndeterminateType);
        }
        let tys = self
            .cases
            .iter()
            .map(|n| n.infer_type(scx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            return Err(LinkedErr::unlinked(E::InvalidIfStatement));
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
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            IfCase::If(con, blk, ..) => {
                con.initialize(scx)?;
                blk.initialize(scx)
            }
            IfCase::Else(blk, ..) => blk.initialize(scx),
        }
    }
}

impl Initialize for If {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.cases.iter().try_for_each(|n| n.initialize(scx))?;
        self.infer_type(scx).map(|_| ())
    }
}
