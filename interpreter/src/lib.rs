mod ast;
mod error;
mod runtime;
#[cfg(test)]
mod tests;
mod utils;

pub(crate) use asttree::*;
pub(crate) use boxed::boxed;
pub(crate) use common::*;
pub(crate) use diagnostics::*;
pub(crate) use error::*;
pub(crate) use lexer::{Keyword, Kind};
pub(crate) use runtime::*;
pub(crate) use semantic::*;
pub(crate) use utils::*;

#[cfg(test)]
pub(crate) use parser::*;
#[cfg(test)]
pub(crate) use semantic::*;

pub(crate) use std::{collections::HashMap, ops::RangeInclusive, path::PathBuf, sync::Arc};
pub(crate) use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
pub(crate) use uuid::Uuid;

pub trait Interpret {
    fn interpret(&self, _rt: Runtime) -> RtPinnedResult<LinkedErr<E>>;
}
