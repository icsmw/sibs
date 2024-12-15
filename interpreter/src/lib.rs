mod ast;
mod error;
mod runtime;

pub(crate) use asttree::*;
pub(crate) use boxed::boxed;
pub(crate) use common::*;
pub(crate) use diagnostics::*;
pub(crate) use error::*;
pub(crate) use runtime::*;

pub trait Interpret {
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>>;
}
