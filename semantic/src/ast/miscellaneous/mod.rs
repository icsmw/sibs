mod comment;
mod include;
mod meta;
mod module;

use crate::*;
use asttree::*;
use diagnostics::*;

impl InferType for Miscellaneous {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Empty)
    }
}
