mod conflict;
mod envs;
mod globals;

mod anchor;
mod component;
mod module;
mod task;

use crate::*;

impl AsVec<RootId> for RootId {
    fn as_vec() -> Vec<RootId> {
        RootId::as_vec()
    }
}

impl TryRead<Root, RootId> for Root {
    fn try_read(parser: &Parser, id: RootId) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        Ok(match id {
            RootId::GlobalsModule => GlobalsModule::read_as_linked(parser)?,
            RootId::EnvsModule => EnvsModule::read_as_linked(parser)?,
            RootId::Anchor => Anchor::read_as_linked(parser)?,
            RootId::Module => Module::read_as_linked(parser)?,
            RootId::Component => Component::read_as_linked(parser)?,
            RootId::Task => Task::read_as_linked(parser)?,
        })
    }
}
