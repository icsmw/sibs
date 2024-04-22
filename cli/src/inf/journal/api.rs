use crate::inf::journal::{Level, Report};
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
    /// Emit shutdown of events loop
    Destroy,
}
