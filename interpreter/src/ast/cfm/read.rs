use crate::*;

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
        nodes: &Nodes,
    ) -> Result<Option<ControlFlowModifier>, E> {
        Ok(match id {
            ControlFlowModifierId::Gatekeeper => {
                Gatekeeper::read(parser, nodes)?.map(ControlFlowModifier::Gatekeeper)
            }
        })
    }
}
