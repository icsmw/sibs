use crate::*;

impl AsVec<MiscellaneousId> for MiscellaneousId {
    fn as_vec() -> Vec<MiscellaneousId> {
        MiscellaneousId::as_vec()
    }
}

impl Read<Miscellaneous, MiscellaneousId> for Miscellaneous {}

impl TryRead<Miscellaneous, MiscellaneousId> for Miscellaneous {
    fn try_read(parser: &mut Parser, id: MiscellaneousId) -> Result<Option<Miscellaneous>, E> {
        Ok(match id {
            MiscellaneousId::Include => Include::read(parser)?.map(Miscellaneous::Include),
            MiscellaneousId::Module => Module::read(parser)?.map(Miscellaneous::Module),
            MiscellaneousId::Comment => Comment::read(parser)?.map(Miscellaneous::Comment),
            MiscellaneousId::Meta => Meta::read(parser)?.map(Miscellaneous::Meta),
        })
    }
}
