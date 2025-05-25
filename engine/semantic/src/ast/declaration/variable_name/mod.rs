use crate::*;

impl InferType for VariableName {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for VariableName {
    fn initialize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

impl Finalization for VariableName {
    fn finalize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

impl SemanticTokensGetter for VariableName {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        match stcx {
            SemanticTokenContext::Ignored | SemanticTokenContext::VariableDeclaration => {
                vec![LinkedSemanticToken::from_token(
                    &self.token,
                    SemanticToken::Variable,
                )]
            }
            SemanticTokenContext::ArgumentDeclaration | SemanticTokenContext::FunctionCall => {
                vec![LinkedSemanticToken::from_token(
                    &self.token,
                    SemanticToken::Parameter,
                )]
            }
        }
    }
}
