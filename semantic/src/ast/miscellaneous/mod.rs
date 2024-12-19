mod comment;
mod meta;

use crate::*;

impl InferType for Miscellaneous {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.infer_type(tcx),
            Miscellaneous::Meta(n) => n.infer_type(tcx),
        }
    }
}

impl Initialize for Miscellaneous {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.initialize(tcx),
            Miscellaneous::Meta(n) => n.initialize(tcx),
        }
    }
}
