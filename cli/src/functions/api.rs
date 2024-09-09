use std::sync::Arc;

use crate::{
    functions::{ExecutorFnDescription, E},
    inf::ValueRef,
};
use tokio::sync::oneshot;

/// Represents API of tast's context. Because each task has own context and
/// multiple tasks could be runned concurrency, communication goes via channels.
pub enum Demand {
    /// Get function description
    ///
    /// # Parameters
    ///
    /// * `String` - Name of function
    /// * `Option<ValueRef>` - The first argument type
    /// * `oneshot::Sender<Result<Arc<ExecutorFnDescription>, E>>` - Response channel with reference
    ///   to function's executor
    GetFunctionDescription(
        String,
        Option<ValueRef>,
        oneshot::Sender<Result<Arc<ExecutorFnDescription>, E>>,
    ),
    /// Emit shutdown of events loop
    Destroy,
}
