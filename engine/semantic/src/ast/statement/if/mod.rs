#[cfg(test)]
mod tests;

use crate::*;

impl InferType for IfCase {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        match self {
            IfCase::If(_, blk, ..) => blk.infer_type(scx),
            IfCase::Else(blk, ..) => blk.infer_type(scx),
        }
    }
}

impl InferType for If {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        if !self.cases.iter().any(|c| matches!(c, IfCase::Else(..))) {
            return Ok(Ty::Indeterminate);
        }
        let tys = self
            .cases
            .iter()
            .map(|n| n.infer_type(scx))
            .collect::<Result<Vec<_>, _>>()?;
        if tys.is_empty() {
            return Err(LinkedErr::from(E::InvalidIfStatement, self));
        };
        let first = &tys[0];
        if tys.iter().all(|ty| first.compatible(ty)) {
            Ok(first.clone())
        } else {
            Ok(Ty::Indeterminate)
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

impl Finalization for IfCase {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            IfCase::If(con, blk, ..) => {
                con.finalize(scx)?;
                blk.finalize(scx)
            }
            IfCase::Else(blk, ..) => blk.finalize(scx),
        }
    }
}

impl SemanticTokensGetter for IfCase {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        match self {
            IfCase::If(con, blk, tk) => {
                let mut tokens = vec![LinkedSemanticToken::from_token(tk, SemanticToken::Keyword)];
                tokens.extend(con.get_semantic_tokens(stcx));
                tokens.extend(blk.get_semantic_tokens(stcx));
                tokens
            }
            IfCase::Else(blk, tk) => {
                let mut tokens = vec![LinkedSemanticToken::from_token(tk, SemanticToken::Keyword)];
                tokens.extend(blk.get_semantic_tokens(stcx));
                tokens
            }
        }
    }
}

impl Initialize for If {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.cases.iter().try_for_each(|n| n.initialize(scx))?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for If {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.cases.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}

impl SemanticTokensGetter for If {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.cases
            .iter()
            .flat_map(|n| n.get_semantic_tokens(stcx))
            .collect()
    }
}
