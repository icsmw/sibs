mod gatekeeper;
mod skip;

use crate::*;

impl Interpret for ControlFlowModifier {
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            ControlFlowModifier::Gatekeeper(n) => n.interpret(rt),
            ControlFlowModifier::Skip(n) => n.interpret(rt),
        }
    }
}
