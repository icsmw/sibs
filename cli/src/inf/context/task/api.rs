use crate::inf::{context::E, AnyValue};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{mpsc::UnboundedSender, oneshot};

/// Represents API of tast's context. Because each task has own context and
/// multiple tasks could be runned concurrency, communication goes via channels.
pub enum Demand {
    /// Setting variable value
    ///
    /// # Parameters
    ///
    /// * `String` - Key/Name of variable
    /// * `AnyValue` - Variable value
    /// * `oneshot::Sender<bool>` - Response channel. True - if value replaced; false - if not
    SetVariable(String, AnyValue, oneshot::Sender<bool>),
    /// Getting variable value
    ///
    /// # Parameters
    ///
    /// * `String` - Key/Name of variable
    /// * `AnyValue` - Variable value
    /// * `oneshot::Sender<Option<Arc<AnyValue>>>` - Response channel to return variable value if it's available
    GetVariable(String, oneshot::Sender<Option<Arc<AnyValue>>>),
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
}

/// Represents API of tast's context.
#[derive(Clone, Debug)]
pub struct Coupling {
    tx: UnboundedSender<Demand>,
}

impl Coupling {
    /// Setting variable value
    ///
    /// # Arguments
    ///
    /// * `key` - Key/Name of variable
    /// * `value` - Variable value
    ///
    /// # Returns
    ///
    /// `Ok(bool)` true - if value replaced; false - if not, or `Err(E)` in case
    /// of any channel related error
    pub async fn set_variable(&self, key: String, value: AnyValue) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::SetVariable(key, value, tx))?;
        Ok(rx.await?)
    }
    /// Getting variable value
    ///
    /// # Arguments
    ///
    /// * `key` - Key/Name of variable
    ///
    /// # Returns
    ///
    /// `Ok(Option<Arc<AnyValue>>)` with variable value (None - if variable isn't set),
    /// or `Err(E)` in case of any channel related error
    pub async fn get_variable(&self, key: String) -> Result<Option<Arc<AnyValue>>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::GetVariable(key, tx))?;
        Ok(rx.await?)
    }
    /// Setting cwd (current working folder)
    ///
    /// # Arguments
    ///
    /// * `cwd` - it can be None, because tasks without context are allowed
    ///
    /// # Returns
    ///
    /// `Ok(())` in case of success, or `Err(E)` in case of any channel related error
    pub async fn set_cwd(&self, cwd: Option<PathBuf>) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::SetCwd(cwd, tx))?;
        Ok(rx.await?)
    }
    /// Getting cwd (current working folder)
    ///
    /// # Returns
    ///
    /// `Ok(Option<PathBuf>)` with current working folder (None - if isn't set), or
    /// `Err(E)` in case of any channel related error
    pub async fn get_cwd(&self) -> Result<Option<PathBuf>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::GetCwd(tx))?;
        Ok(rx.await?)
    }
}
