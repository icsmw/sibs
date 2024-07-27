use crate::inf::AnyValue;
use std::{collections::HashMap, fmt, path::PathBuf, sync::Arc};
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

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
    ///   be overwritten in anyway.
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
    /// Set current loop signature and returns UUID of it. Setting loop allows to send
    /// break signal and detect target loop to be stopped
    ///
    /// # Parameters
    ///
    /// * `oneshot::Sender<(Uuid, CancellationToken)>` - Response channel. Uuid of loop
    ///   and CancellationToken to check a state of loop
    OpenLoop(oneshot::Sender<(Uuid, CancellationToken)>),
    /// Close loop.
    ///
    /// # Parameters
    /// * `Uuid` - UUID of target loop
    /// * `oneshot::Sender<()>` - Response channel
    CloseLoop(Uuid, oneshot::Sender<()>),
    /// Breaks current loop if exists.
    ///
    /// # Parameters
    /// * `oneshot::Sender<()>` - Response channel. Returns true if break-signal has
    ///   been sent
    BreakLoop(oneshot::Sender<bool>),
    /// Emit shutdown of events loop
    Destroy,
}

impl fmt::Display for Demand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SetVariable(..) => "SetVariable",
                Self::SetCwd(..) => "SetCwd",
                Self::BreakLoop(..) => "BreakLoop",
                Self::CloseLoop(..) => "CloseLoop",
                Self::Destroy => "Destroy",
                Self::GetCwd(..) => "GetCwd",
                Self::GetVariable(..) => "GetVariable",
                Self::GetVariables(..) => "GetVariables",
                Self::OpenLoop(..) => "OpenLoop",
            }
        )
    }
}
