mod gatekeeper;
mod skip;

use crate::*;

impl Interpret for ControlFlowModifier {
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            ControlFlowModifier::Gatekeeper(n) => n.interpret(rt, cx),
            ControlFlowModifier::Skip(n) => n.interpret(rt, cx),
        }
    }
}
