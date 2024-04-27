use crate::inf::journal::{Level, Report};
use tokio::sync::oneshot;
use uuid::Uuid;

/// Represents API of LifeCycle.
#[derive(Debug)]
pub enum Demand {
    /// Adds log message
    ///
    /// # Parameters
    ///
    /// * `String` - Owner/sender of logs
    /// * `String` - Log message
    /// * `Level` - Level of log
    Log(String, String, Level),
    /// Adds report
    ///
    /// # Parameters
    ///
    /// * `String` - Report as a string
    Report(Report),
    /// Mark report as tolerant
    ///
    /// # Parameters
    ///
    /// * `Uuid` - Uuid of error (bound with error)
    Toleranted(Uuid),
    /// Collecting logs without posting
    ///
    /// # Parameters
    ///
    /// * `usize` - id of thread/job
    /// * `String` - message to collect (will be append to already collected)
    Collect(usize, String),
    /// Finish collecting logs for given thread/job. Depending on configuration
    /// logs will be posted or not
    ///
    /// # Parameters
    ///
    /// * `String` - Owner/sender of logs
    /// * `usize` - id of thread/job
    /// * `Level` - level of logs
    CollectionClose(String, usize, Level),
    /// Reads all message until this
    ///
    /// # Parameters
    ///
    /// * `oneshot::Sender<()>` - Channel to response
    Flush(oneshot::Sender<()>),
    /// Emit shutdown of events loop
    Destroy,
}
