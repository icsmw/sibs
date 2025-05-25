mod comment;
mod meta;

use crate::*;

impl InferType for Miscellaneous {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.infer_type(scx),
            Miscellaneous::Meta(n) => n.infer_type(scx),
        }
    }
}

impl Initialize for Miscellaneous {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.initialize(scx),
            Miscellaneous::Meta(n) => n.initialize(scx),
        }
    }
}

impl Finalization for Miscellaneous {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.finalize(scx),
            Miscellaneous::Meta(n) => n.finalize(scx),
        }
    }
}

impl SemanticTokensGetter for Miscellaneous {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        match self {
            Miscellaneous::Comment(n) => n.get_semantic_tokens(stcx),
            Miscellaneous::Meta(n) => n.get_semantic_tokens(stcx),
        }
    }
}
