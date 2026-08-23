mod gatekeeper;
mod skip;

use crate::*;

impl Interpret for ControlFlowModifier {
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            ControlFlowModifier::Gatekeeper(n) => n.interpret(env),
            ControlFlowModifier::Skip(n) => n.interpret(env),
        }
    }
}
