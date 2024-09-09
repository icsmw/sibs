use crate::inf::{context::E, Value};
use std::{fmt, path::PathBuf, sync::Arc};
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

/// Represents API of tast's context. Because each task has own context and
/// multiple tasks could be runned concurrency, communication goes via channels.
pub enum Demand {
    AddSession(String, Option<PathBuf>, oneshot::Sender<Uuid>),
    RemoveSession(Uuid, oneshot::Sender<()>),
    /// Setting global variable value
    ///
    /// # Parameters
    ///
    /// * `String` - Key/Name of variable
    /// * `Value` - Variable value
    ///   be overwritten in anyway.
    /// * `oneshot::Sender<bool>` - Response channel. True - if value replaced; false - if not
    SetGlobalVariable(String, Value, oneshot::Sender<Result<bool, E>>),
    /// Getting global variable value
    ///
    /// # Parameters
    ///
    /// * `String` - Key/Name of variable
    /// * `oneshot::Sender<Option<Arc<Value>>>` - Response channel to return variable value if it's available
    GetGlobalVariable(String, oneshot::Sender<Result<Option<Arc<Value>>, E>>),
    /// Setting variable value
    ///
    /// # Parameters
    ///
    /// * `Uuid` - Uuid of local scope (context of component/task)
    /// * `String` - Key/Name of variable
    /// * `Value` - Variable value
    ///   be overwritten in anyway.
    /// * `oneshot::Sender<bool>` - Response channel. True - if value replaced; false - if not
    SetVariable(Uuid, String, Value, oneshot::Sender<Result<bool, E>>),
    /// Getting variable value
    ///
    /// # Parameters
    ///
    /// * `Uuid` - Uuid of local scope (context of component/task)
    /// * `String` - Key/Name of variable
    /// * `oneshot::Sender<Option<Arc<Value>>>` - Response channel to return variable value if it's available
    GetVariable(Uuid, String, oneshot::Sender<Result<Option<Arc<Value>>, E>>),
    /// Getting current working folder for task
    ///
    /// # Parameters
    ///
    /// * `Uuid` - Uuid of local scope (context of component/task)
    /// * `oneshot::Sender<PathBuf>` - Response channel
    GetCwd(Uuid, oneshot::Sender<Result<PathBuf, E>>),
    /// Getting global working folder for task (match with scenario path)
    ///
    /// # Parameters
    ///
    /// * `oneshot::Sender<PathBuf>` - Response channel
    GetGlobalCwd(oneshot::Sender<Result<PathBuf, E>>),
    /// Setting current working folder for task
    ///
    /// # Parameters
    ///
    /// * `Uuid` - Uuid of local scope (context of component/task)
    /// * `PathBuf` - Current working folder of task
    /// * `oneshot::Sender<()>` - Response channel
    SetCwd(Uuid, PathBuf, oneshot::Sender<Result<(), E>>),
    /// Set current loop signature and returns UUID of it. Setting loop allows to send
    /// break signal and detect target loop to be stopped
    ///
    /// # Parameters
    ///
    /// * `Uuid` - Uuid of local scope (context of component/task)
    /// * `oneshot::Sender<(Uuid, CancellationToken)>` - Response channel. Uuid of loop
    ///   and CancellationToken to check a state of loop
    OpenLoop(Uuid, oneshot::Sender<Result<(Uuid, CancellationToken), E>>),
    /// Close loop.
    ///
    /// # Parameters
    ///
    /// * `Uuid` - Uuid of local scope (context of component/task)
    /// * `Uuid` - UUID of target loop
    /// * `oneshot::Sender<()>` - Response channel
    CloseLoop(Uuid, Uuid, oneshot::Sender<Result<(), E>>),
    /// Breaks current loop if exists.
    ///
    /// # Parameters
    ///
    /// * `Uuid` - Uuid of local scope (context of component/task)
    /// * `oneshot::Sender<()>` - Response channel. Returns true if break-signal has
    ///   been sent
    BreakLoop(Uuid, oneshot::Sender<Result<bool, E>>),
    /// Emit shutdown of events loop
    Destroy,
}

impl fmt::Display for Demand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SetGlobalVariable(..) => "SetGlobalVariable",
                Self::GetGlobalVariable(..) => "GetGlobalVariable",
                Self::RemoveSession(..) => "RemoveSession",
                Self::AddSession(..) => "AddSession",
                Self::SetVariable(..) => "SetVariable",
                Self::SetCwd(..) => "SetCwd",
                Self::BreakLoop(..) => "BreakLoop",
                Self::CloseLoop(..) => "CloseLoop",
                Self::Destroy => "Destroy",
                Self::GetCwd(..) => "GetCwd",
                Self::GetGlobalCwd(..) => "GetGlobalCwd",
                Self::GetVariable(..) => "GetVariable",
                Self::OpenLoop(..) => "OpenLoop",
            }
        )
    }
}
