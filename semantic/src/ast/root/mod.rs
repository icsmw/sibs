mod component;
mod task;

use crate::*;
use asttree::*;
use diagnostics::*;

impl InferType for Root {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Empty)
    }
}
