mod conflict;

mod gatekeeper;
mod skip;

use crate::*;

impl AsVec<ControlFlowModifierId> for ControlFlowModifierId {
    fn as_vec() -> Vec<ControlFlowModifierId> {
        ControlFlowModifierId::as_vec()
    }
}

impl TryRead<ControlFlowModifier, ControlFlowModifierId> for ControlFlowModifier {
    fn try_read(
        parser: &mut Parser,
        id: ControlFlowModifierId,
    ) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        Ok(match id {
            ControlFlowModifierId::Gatekeeper => Gatekeeper::read_as_linked(parser)?,
            ControlFlowModifierId::Skip => Skip::read_as_linked(parser)?,
        })
    }
}
