mod ast;
mod error;
mod runtime;
mod utils;

pub(crate) use asttree::*;
pub(crate) use boxed::boxed;
pub(crate) use common::*;
pub(crate) use diagnostics::*;
pub(crate) use error::*;
pub(crate) use runtime::*;
pub(crate) use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    oneshot,
};
pub(crate) use utils::*;
pub(crate) use uuid::Uuid;

pub trait Interpret {
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>>;
}
