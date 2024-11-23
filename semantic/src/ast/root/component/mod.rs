use crate::*;
use asttree::*;
use diagnostics::*;

impl InferType for Component {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Empty)
    }
}
