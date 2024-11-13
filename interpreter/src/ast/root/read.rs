use crate::*;

impl AsVec<RootId> for RootId {
    fn as_vec() -> Vec<RootId> {
        RootId::as_vec()
    }
}

impl Read<Root, RootId> for Root {}

impl TryRead<Root, RootId> for Root {
    fn try_read(parser: &mut Parser, id: RootId) -> Result<Option<Root>, E> {
        Ok(match id {
            RootId::Component => Component::read(parser)?.map(Root::Component),
            RootId::Task => Task::read(parser)?.map(Root::Task),
        })
    }
}
