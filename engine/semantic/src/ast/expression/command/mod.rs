use crate::*;

impl InferType for Command {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::ExecuteResult.into())
    }
}

impl Initialize for CommandPart {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let CommandPart::Expression(n) = self {
            n.initialize(scx)
        } else {
            Ok(())
        }
    }
}

impl Finalization for CommandPart {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let CommandPart::Expression(n) = self {
            n.finalize(scx)
        } else {
            Ok(())
        }
    }
}

impl SemanticTokensGetter for CommandPart {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        match self {
            CommandPart::Open(tk) | CommandPart::Close(tk) => {
                vec![LinkedSemanticToken::from_token(
                    tk,
                    SemanticToken::Delimiter,
                )]
            }
            CommandPart::Expression(n) => n.get_semantic_tokens(stcx),
            CommandPart::Literal(tk) => {
                vec![LinkedSemanticToken::from_token(tk, SemanticToken::String)]
            }
        }
    }
}

impl Initialize for Command {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for Command {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}

impl SemanticTokensGetter for Command {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.nodes
            .iter()
            .flat_map(|n| n.get_semantic_tokens(stcx))
            .collect()
    }
}
