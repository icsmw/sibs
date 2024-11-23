mod comment;
mod include;
mod meta;
mod module;

use crate::*;


impl InferType for Miscellaneous {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            Miscellaneous::Comment(n) => n.infer_type(tcx),
            Miscellaneous::Include(n) => n.infer_type(tcx),
            Miscellaneous::Meta(n) => n.infer_type(tcx),
            Miscellaneous::Module(n) => n.infer_type(tcx),
        }
    }
}
