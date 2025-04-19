mod conflict;

mod comment;
mod meta;

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
            MiscellaneousId::Comment => Comment::read_as_linked(parser)?,
            MiscellaneousId::Meta => Meta::read_as_linked(parser)?,
        })
    }
}
