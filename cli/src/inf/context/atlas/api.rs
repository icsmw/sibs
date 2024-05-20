use crate::{error::LinkedErrSerialized, inf::context::atlas::E};
use std::fmt;
use tokio::sync::oneshot;

/// Represents API of tast's context. Because each task has own context and
/// multiple tasks could be runned concurrency, communication goes via channels.
pub enum Demand {
    /// Add footprint into trace
    ///
    /// # Parameters
    ///
    /// * `usize` - Token
    /// * `Option<String>` - String representation of value; None if value isn't set
    /// * `oneshot::Sender<()>` - Response channel
    AddFootprint(usize, Option<String>, oneshot::Sender<Result<(), E>>),
    /// Add error report into trace
    ///
    /// # Parameters
    ///
    /// * `LinkedErrSerialized` - Related error
    /// * `oneshot::Sender<()>` - Response channel
    ReportErr(LinkedErrSerialized, oneshot::Sender<Result<(), E>>),
    /// Set position of cursor in current map
    ///
    /// # Parameters
    ///
    /// * `usize` - Token
    /// * `oneshot::Sender<()>` - Response channel
    SetMapPosition(usize, oneshot::Sender<Result<(), E>>),
    /// Emit shutdown of events loop
    Destroy,
}

impl fmt::Display for Demand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AddFootprint(..) => "AddFootprint",
                Self::Destroy => "Destroy",
                Self::ReportErr(..) => "ReportErr",
                Self::SetMapPosition(..) => "SetMapPosition",
            }
        )
    }
}
