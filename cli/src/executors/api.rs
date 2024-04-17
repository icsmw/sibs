use crate::{
    executors::ExecutorResult,
    inf::{AnyValue, Context},
};
use tokio::sync::oneshot;

/// Represents API of tast's context. Because each task has own context and
/// multiple tasks could be runned concurrency, communication goes via channels.
pub enum Demand {
    /// Execute function
    ///
    /// # Parameters
    ///
    /// * `String` - Name of function
    /// * `AnyValue` - Function argument
    /// * `oneshot::Sender<ExecutorResult>` - Response channel with result of executing
    Execute(
        String,
        Vec<AnyValue>,
        Context,
        oneshot::Sender<ExecutorResult>,
    ),
    /// Emit shutdown of events loop
    Destroy,
}
