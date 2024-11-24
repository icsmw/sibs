mod gatekeeper;
mod skip;

use crate::*;

impl InferType for ControlFlowModifier {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            ControlFlowModifier::Gatekeeper(n) => n.infer_type(tcx),
            ControlFlowModifier::Skip(n) => n.infer_type(tcx),
        }
    }
}

impl Initialize for ControlFlowModifier {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        match self {
            ControlFlowModifier::Gatekeeper(n) => n.initialize(tcx),
            ControlFlowModifier::Skip(n) => n.initialize(tcx),
        }
    }
}
