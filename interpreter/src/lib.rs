mod ast;
mod error;

pub(crate) use asttree::*;
pub(crate) use boxed::boxed;
pub(crate) use diagnostics::*;
pub(crate) use error::*;
pub(crate) use lexer::{Keyword, Kind};
pub(crate) use runtime::*;
pub(crate) use tokio::sync::oneshot;
pub(crate) use uuid::Uuid;

pub trait Interpret {
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>>;
}
