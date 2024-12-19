mod anchor;
mod component;
mod module;
mod task;

use crate::*;

impl InferType for Root {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            Root::Task(n) => n.infer_type(tcx),
            Root::Component(n) => n.infer_type(tcx),
            Root::Module(n) => n.infer_type(tcx),
            Root::Anchor(n) => n.infer_type(tcx),
        }
    }
}

impl Initialize for Root {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        match self {
            Root::Task(n) => n.initialize(tcx),
            Root::Component(n) => n.initialize(tcx),
            Root::Module(n) => n.initialize(tcx),
            Root::Anchor(n) => n.initialize(tcx),
        }
    }
}
