use crate::inf::AnyValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::oneshot;

/// Represents API of tast's context. Because each task has own context and
/// multiple tasks could be runned concurrency, communication goes via channels.
pub enum Demand {
    /// Setting variable value
    ///
    /// # Parameters
    ///
    /// * `String` - Key/Name of variable
    /// * `AnyValue` - Variable value
    /// * `bool` - true - log warning if variable exists; false - do not log any; variable will
    /// be overwritten in anyway.
    /// * `oneshot::Sender<bool>` - Response channel. True - if value replaced; false - if not
    SetVariable(String, AnyValue, bool, oneshot::Sender<bool>),
    /// Getting variable value
    ///
    /// # Parameters
    ///
    /// * `String` - Key/Name of variable
    /// * `oneshot::Sender<Option<Arc<AnyValue>>>` - Response channel to return variable value if it's available
    GetVariable(String, oneshot::Sender<Option<Arc<AnyValue>>>),
    /// Getting all variables
    ///
    /// # Parameters
    ///
    /// * `AnyValue` - Variable value
    /// * `oneshot::Sender<HashMap<String, Arc<AnyValue>>>` - Response channel to return variables
    GetVariables(oneshot::Sender<HashMap<String, Arc<AnyValue>>>),
    /// Getting current working folder for task
    ///
    /// # Parameters
    ///
    /// * `oneshot::Sender<Option<PathBuf>>` - Response channel
    GetCwd(oneshot::Sender<Option<PathBuf>>),
    /// Setting current working folder for task
    ///
    /// # Parameters
    ///
    /// * `Option<PathBuf>` - Current working folder of task
    /// * `oneshot::Sender<()>` - Response channel
    SetCwd(Option<PathBuf>, oneshot::Sender<()>),
    /// Emit shutdown of events loop
    Destroy,
}
