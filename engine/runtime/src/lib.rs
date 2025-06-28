mod declaration;
pub mod error;
mod rt;
pub mod spawner;
mod utils;
mod value;

pub(crate) use asttree::*;
pub use declaration::*;
pub(crate) use diagnostics::*;
pub use error::E as RtError;
pub(crate) use error::*;
pub(crate) use lexer::*;
pub use rt::*;
pub use value::*;

pub(crate) use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    path::PathBuf,
    sync::Arc,
};
pub(crate) use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        oneshot,
    },
};
pub(crate) use uuid::Uuid;

pub(crate) const SIBS_FOLDER: &str = ".sibs";
pub(crate) const STORAGE_FOLDER: &str = "storage";
pub(crate) const SESSIONS_FILENAME: &str = "sessions.brec";
