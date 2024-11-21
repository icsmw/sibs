mod conflict;
mod interest;
mod link;

mod gatekeeper;
mod skip;

use crate::*;
use asttree::*;

impl AsVec<ControlFlowModifierId> for ControlFlowModifierId {
    fn as_vec() -> Vec<ControlFlowModifierId> {
        ControlFlowModifierId::as_vec()
    }
}

impl Read<ControlFlowModifier, ControlFlowModifierId> for ControlFlowModifier {}

impl TryRead<ControlFlowModifier, ControlFlowModifierId> for ControlFlowModifier {
    fn try_read(
        parser: &mut Parser,
        id: ControlFlowModifierId,
    ) -> Result<Option<ControlFlowModifier>, LinkedErr<E>> {
        Ok(match id {
            ControlFlowModifierId::Gatekeeper => {
                Gatekeeper::read(parser)?.map(ControlFlowModifier::Gatekeeper)
            }
            ControlFlowModifierId::Skip => Skip::read(parser)?.map(ControlFlowModifier::Skip),
        })
    }
}
