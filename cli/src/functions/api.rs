use std::sync::Arc;

use crate::{
    functions::{ExecutorFn, E},
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
    /// * `oneshot::Sender<Result<Arc<ExecutorFn>, E>>` - Response channel with reference
    /// to function's executor
    Execute(String, oneshot::Sender<Result<Arc<ExecutorFn>, E>>),
    /// Emit shutdown of events loop
    Destroy,
}
