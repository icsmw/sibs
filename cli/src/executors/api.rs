use crate::{
    executors::ExecutorResult,
    inf::{AnyValue, Context, Scope},
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
    /// * `Context` - Global context
    /// * `Scope` - Task's scope
    /// * `oneshot::Sender<ExecutorResult>` - Response channel with result of executing
    Execute(
        String,
        Vec<AnyValue>,
        Context,
        Scope,
        oneshot::Sender<ExecutorResult>,
    ),
    /// Emit shutdown of events loop
    Destroy,
}
