use std::{collections::HashMap, sync::Arc};

use crate::functions::{ExecutorFnDescription, E};
use tokio::sync::oneshot;

/// Represents API of tast's context. Because each task has own context and
/// multiple tasks could be runned concurrency, communication goes via channels.
pub enum Demand {
    /// Get function description
    ///
    /// # Parameters
    ///
    /// * `String` - Name of function
    /// * `oneshot::Sender<Result<Arc<ExecutorFnDescription>, E>>` - Response channel with reference
    ///   to function's executor
    GetFunctionDescription(
        String,
        oneshot::Sender<Result<Arc<ExecutorFnDescription>, E>>,
    ),
    /// Emit shutdown of events loop
    Destroy,
    #[cfg(test)]
    GetAll(oneshot::Sender<HashMap<String, Arc<ExecutorFnDescription>>>),
}
