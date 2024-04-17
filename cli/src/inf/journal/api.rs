use crate::inf::journal::{Level, Report};

/// Represents API of LifeCycle.
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
    /// Emit shutdown of events loop
    Destroy,
}
