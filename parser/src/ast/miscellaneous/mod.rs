mod conflict;

mod comment;
mod include;
mod meta;
mod module;

use crate::*;

impl AsVec<MiscellaneousId> for MiscellaneousId {
    fn as_vec() -> Vec<MiscellaneousId> {
        MiscellaneousId::as_vec()
    }
}

impl TryRead<Miscellaneous, MiscellaneousId> for Miscellaneous {
    fn try_read(
        parser: &mut Parser,
        id: MiscellaneousId,
    ) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        Ok(match id {
            MiscellaneousId::Include => Include::read_as_linked(parser)?,
            MiscellaneousId::Module => Module::read_as_linked(parser)?,
            MiscellaneousId::Comment => Comment::read_as_linked(parser)?,
            MiscellaneousId::Meta => Meta::read_as_linked(parser)?,
        })
    }
}
