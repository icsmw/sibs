mod ast;
mod error;
#[cfg(test)]
mod tests;
mod utils;

pub(crate) use asttree::*;
pub(crate) use boxed::boxed;
pub(crate) use diagnostics::*;
pub(crate) use error::*;
pub(crate) use lexer::{Keyword, Kind};
pub(crate) use runtime::*;
pub(crate) use tokio::sync::oneshot;
pub(crate) use utils::*;
pub(crate) use uuid::Uuid;

#[cfg(test)]
pub(crate) use parser::*;
#[cfg(test)]
pub(crate) use semantic::*;

pub trait Interpret {
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>>;
}
