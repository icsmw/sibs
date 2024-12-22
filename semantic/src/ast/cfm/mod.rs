mod gatekeeper;
mod skip;

use crate::*;

impl InferType for ControlFlowModifier {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        match self {
            ControlFlowModifier::Gatekeeper(n) => n.infer_type(scx),
            ControlFlowModifier::Skip(n) => n.infer_type(scx),
        }
    }
}

impl Initialize for ControlFlowModifier {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            ControlFlowModifier::Gatekeeper(n) => n.initialize(scx),
            ControlFlowModifier::Skip(n) => n.initialize(scx),
        }
    }
}

impl Finalization for ControlFlowModifier {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            ControlFlowModifier::Gatekeeper(n) => n.finalize(scx),
            ControlFlowModifier::Skip(n) => n.finalize(scx),
        }
    }
}
