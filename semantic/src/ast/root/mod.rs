mod anchor;
mod component;
mod module;
mod task;

use crate::*;

impl InferType for Root {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
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
