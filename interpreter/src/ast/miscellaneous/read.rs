use crate::*;

impl AsVec<MiscellaneousId> for MiscellaneousId {
    fn as_vec() -> Vec<MiscellaneousId> {
        MiscellaneousId::as_vec()
    }
}

impl Read<Miscellaneous, MiscellaneousId> for Miscellaneous {}

impl TryRead<Miscellaneous, MiscellaneousId> for Miscellaneous {
    fn try_read(
        parser: &mut Parser,
        id: MiscellaneousId,
        nodes: &Nodes,
    ) -> Result<Option<Miscellaneous>, E> {
        Ok(match id {
            MiscellaneousId::Command => Command::read(parser, nodes)?.map(Miscellaneous::Command),
        })
    }
}
