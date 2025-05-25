use crate::*;

impl InferType for InterpolatedString {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Str.into())
    }
}

impl Initialize for InterpolatedStringPart {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let InterpolatedStringPart::Expression(n) = self {
            n.initialize(scx)
        } else {
            Ok(())
        }
    }
}

impl Finalization for InterpolatedStringPart {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let InterpolatedStringPart::Expression(n) = self {
            n.finalize(scx)
        } else {
            Ok(())
        }
    }
}

impl SemanticTokensGetter for InterpolatedStringPart {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        match self {
            InterpolatedStringPart::Open(tk) | InterpolatedStringPart::Close(tk) => {
                vec![LinkedSemanticToken::from_token(
                    tk,
                    SemanticToken::Delimiter,
                )]
            }
            InterpolatedStringPart::Expression(n) => n.get_semantic_tokens(stcx),
            InterpolatedStringPart::Literal(tk) => {
                vec![LinkedSemanticToken::from_token(tk, SemanticToken::String)]
            }
        }
    }
}

impl Initialize for InterpolatedString {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for InterpolatedString {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}

impl SemanticTokensGetter for InterpolatedString {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.nodes
            .iter()
            .flat_map(|n| n.get_semantic_tokens(stcx))
            .collect()
    }
}
