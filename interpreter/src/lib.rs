mod ast;
mod utils;

#[cfg(test)]
mod tests;

pub(crate) use asttree::*;
pub(crate) use boxed::boxed;
pub(crate) use diagnostics::*;
pub(crate) use lexer::{Keyword, Kind};
pub(crate) use runtime::error::E;
pub(crate) use runtime::*;
pub(crate) use utils::*;

#[cfg(test)]
pub(crate) use parser::*;
#[cfg(test)]
pub(crate) use semantic::*;

pub trait Interpret {
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>>;
}
