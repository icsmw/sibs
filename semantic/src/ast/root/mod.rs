mod component;
mod task;

use crate::*;


impl InferType for Root {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            Root::Task(n) => n.infer_type(tcx),
            Root::Component(n) => n.infer_type(tcx),
        }
    }
}
