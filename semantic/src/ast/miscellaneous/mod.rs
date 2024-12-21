mod comment;
mod meta;

use crate::*;

impl InferType for Miscellaneous {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
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
