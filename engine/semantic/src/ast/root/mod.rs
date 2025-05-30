mod anchor;
mod component;
mod module;
mod task;

use crate::*;

impl InferType for Root {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        match self {
            Root::Task(n) => n.infer_type(scx),
            Root::Component(n) => n.infer_type(scx),
            Root::Module(n) => n.infer_type(scx),
            Root::Anchor(n) => n.infer_type(scx),
        }
    }
}

impl Initialize for Root {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Root::Task(n) => n.initialize(scx),
            Root::Component(n) => n.initialize(scx),
            Root::Module(n) => n.initialize(scx),
            Root::Anchor(n) => n.initialize(scx),
        }
    }
}

impl Finalization for Root {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Root::Task(n) => n.finalize(scx),
            Root::Component(n) => n.finalize(scx),
            Root::Module(n) => n.finalize(scx),
            Root::Anchor(n) => n.finalize(scx),
        }
    }
}

impl SemanticTokensGetter for Root {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        match self {
            Root::Task(n) => n.get_semantic_tokens(stcx),
            Root::Component(n) => n.get_semantic_tokens(stcx),
            Root::Module(n) => n.get_semantic_tokens(stcx),
            Root::Anchor(n) => n.get_semantic_tokens(stcx),
        }
    }
}
