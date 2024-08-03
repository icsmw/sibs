use crate::inf::{context::E, scope::Demand, AnyValue, OwnedJournal};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{mpsc::UnboundedSender, oneshot};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Scope {
    tx: UnboundedSender<Demand>,
    uuid: Uuid,
    pub journal: OwnedJournal,
}

impl Scope {
    pub fn new(tx: UnboundedSender<Demand>, uuid: Uuid, journal: OwnedJournal) -> Self {
        Self { tx, uuid, journal }
    }
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
    pub async fn set_var(&self, key: &str, value: AnyValue) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::SetVariable(self.uuid, key.to_owned(), value, tx))?;
        rx.await?
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
    pub async fn get_var(&self, key: &str) -> Result<Option<Arc<AnyValue>>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::GetVariable(self.uuid, key.to_owned(), tx))?;
        rx.await?
    }

    /// Setting global variable value
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
    pub async fn set_global_var(&self, key: &str, value: AnyValue) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::SetGlobalVariable(key.to_owned(), value, tx))?;
        rx.await?
    }

    /// Getting global variable value
    ///
    /// # Arguments
    ///
    /// * `key` - Key/Name of variable
    ///
    /// # Returns
    ///
    /// `Ok(Option<Arc<AnyValue>>)` with variable value (None - if variable isn't set),
    /// or `Err(E)` in case of any channel related error
    pub async fn get_global_var(&self, key: &str) -> Result<Option<Arc<AnyValue>>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::GetGlobalVariable(key.to_owned(), tx))?;
        rx.await?
    }

    /// Import all variables from given scope. Post warn logs if some variable would
    /// be overwriten because exists on destination scope
    ///
    /// # Arguments
    ///
    /// * `src` - source scope to import variables from
    pub async fn import_vars(&self, src: &Scope) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::ImportVars(self.uuid, src.uuid, tx))?;
        rx.await?
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
    pub async fn set_cwd(&self, cwd: PathBuf) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::SetCwd(self.uuid, cwd, tx))?;
        rx.await?
    }

    /// Getting cwd (current working folder)
    ///
    /// # Returns
    ///
    /// `Ok(Option<PathBuf>)` with current working folder (None - if isn't set), or
    /// `Err(E)` in case of any channel related error
    pub async fn get_cwd(&self) -> Result<PathBuf, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::GetCwd(self.uuid, tx))?;
        rx.await?
    }

    /// Getting global cwd (same as scenario path)
    ///
    /// # Returns
    ///
    /// `Ok(Option<PathBuf>)` with current working folder (None - if isn't set), or
    /// `Err(E)` in case of any channel related error
    pub async fn get_global_cwd(&self) -> Result<PathBuf, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::GetGlobalCwd(tx))?;
        rx.await?
    }

    /// Opening loop in current scope. It's needed only to manage breaking of loop
    ///
    /// # Returns
    ///
    /// `Ok((Uuid, CancellationToken))` Uuid of registred loop and cancellation token
    /// to track state of loop
    pub async fn open_loop(&self) -> Result<(Uuid, CancellationToken), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::OpenLoop(self.uuid, tx))?;
        rx.await?
    }

    /// Close opened loop and send cancel signal to token
    ///
    /// # Arguments
    ///
    /// * `uuid` - Uuid of target loop
    pub async fn close_loop(&self, uuid: Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::CloseLoop(self.uuid, uuid, tx))?;
        rx.await?
    }

    /// Breaks current loop if exists.
    ///
    /// # Returns
    ///
    /// `Ok(bool)` true if break-signal has been sent
    /// to track state of loop
    pub async fn break_loop(&self) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::BreakLoop(self.uuid, tx))?;
        rx.await?
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::RemoveSession(self.uuid, tx))?;
        Ok(rx.await?)
    }
}
