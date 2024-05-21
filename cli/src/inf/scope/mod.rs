mod api;

use crate::inf::{context::E, AnyValue, Journal};
use api::*;
use std::{
    sync::Arc,
    {collections::HashMap, path::PathBuf},
};
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

enum Action {
    Check(bool),
    Break,
    Nothing,
}

#[derive(Debug, Clone)]
pub struct Scope {
    tx: UnboundedSender<api::Demand>,
    state: CancellationToken,
    path: String,
}

impl Scope {
    pub fn init(mut cwd: Option<PathBuf>, journal: &Journal) -> Self {
        let (tx, mut rx): (UnboundedSender<api::Demand>, UnboundedReceiver<api::Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let mut loops: Vec<(Uuid, CancellationToken)> = Vec::new();
        let instance = Self {
            tx,
            state: state.clone(),
            path: String::new(),
        };
        let journal = journal.owned("Scope".to_owned(), None);
        journal.info(format!(
            "initial CWD: {}",
            cwd.as_ref()
                .map(|cwd| cwd.to_string_lossy().to_string())
                .unwrap_or("no CWD context".to_string())
        ));
        spawn(async move {
            let mut vars: HashMap<String, Arc<AnyValue>> = HashMap::new();
            while let Some(demand) = rx.recv().await {
                let requested = demand.to_string();
                let action = match demand {
                    api::Demand::SetVariable(k, v, warn, tx) => {
                        if warn && vars.contains_key(&k) {
                            journal.warn(format!(
                                "Variable \"{k}\" will be overwritten with new value"
                            ));
                        }
                        Action::Check(tx.send(vars.insert(k, Arc::new(v)).is_some()).is_err())
                    }
                    api::Demand::GetVariable(k, tx) => {
                        Action::Check(tx.send(vars.get(&k).cloned()).is_err())
                    }
                    api::Demand::GetVariables(tx) => Action::Check(tx.send(vars.clone()).is_err()),
                    api::Demand::SetCwd(path, tx) => {
                        cwd = path;
                        journal.info(format!(
                            "set CWD to: {}",
                            cwd.as_ref()
                                .map(|cwd| cwd.to_string_lossy().to_string())
                                .unwrap_or("no CWD context".to_string())
                        ));
                        Action::Check(tx.send(()).is_err())
                    }
                    api::Demand::GetCwd(tx) => Action::Check(tx.send(cwd.clone()).is_err()),
                    api::Demand::OpenLoop(tx) => {
                        let token = CancellationToken::new();
                        let uuid = Uuid::new_v4();
                        loops.push((uuid, token.clone()));
                        Action::Check(tx.send((uuid, token)).is_err())
                    }
                    api::Demand::CloseLoop(uuid, tx) => {
                        loops.iter().for_each(|(id, token)| {
                            if id == &uuid {
                                token.cancel();
                            }
                        });
                        loops.retain(|(id, _)| id != &uuid);
                        Action::Check(tx.send(()).is_err())
                    }
                    api::Demand::BreakLoop(tx) => Action::Check(
                        tx.send(
                            loops
                                .pop()
                                .map(|(_, token)| {
                                    token.cancel();
                                    true
                                })
                                .unwrap_or(false),
                        )
                        .is_err(),
                    ),
                    Demand::Destroy => Action::Break,
                };
                match action {
                    Action::Check(is_err) => {
                        if is_err {
                            journal.err(format!("Fail to send response for \"{requested}\""));
                            break;
                        }
                    }
                    Action::Break => {
                        break;
                    }
                    Action::Nothing => {}
                }
            }
            state.cancel();
        });
        instance
    }

    /// Destroy scope due sending destroy command and waiting for abort confirmation
    pub async fn destroy(&self) -> Result<(), E> {
        self.tx.send(Demand::Destroy)?;
        self.state.cancelled().await;
        Ok(())
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
        self.setting_var(key, value, false).await
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
        self.tx.send(Demand::GetVariable(key.to_owned(), tx))?;
        Ok(rx.await?)
    }

    /// Returns all variables defined in the scope
    async fn get_vars(&self) -> Result<HashMap<String, Arc<AnyValue>>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::GetVariables(tx))?;
        Ok(rx.await?)
    }

    /// Import all variables from given scope. Post warn logs if some variable would
    /// be overwriten because exists on destination scope
    ///
    /// # Arguments
    ///
    /// * `src` - source scope to import variables from
    pub async fn import_vars(&self, src: &Scope) -> Result<(), E> {
        for (key, value) in src.get_vars().await? {
            self.setting_var(&key, value.duplicate(), true).await?;
        }
        Ok(())
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

    /// Opening loop in current scope. It's needed only to manage breaking of loop
    ///
    /// # Returns
    ///
    /// `Ok((Uuid, CancellationToken))` Uuid of registred loop and cancellation token
    /// to track state of loop
    pub async fn open_loop(&self) -> Result<(Uuid, CancellationToken), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::OpenLoop(tx))?;
        Ok(rx.await?)
    }

    /// Close opened loop and send cancel signal to token
    ///
    /// # Arguments
    ///
    /// * `uuid` - Uuid of target loop
    pub async fn close_loop(&self, uuid: Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::CloseLoop(uuid, tx))?;
        Ok(rx.await?)
    }

    /// Sets a fullname  of current task. This command is used for bind logs with
    /// current task
    ///
    /// # Arguments
    ///
    /// * `fullname` - fullname of task
    pub fn set_current_task(&mut self, fullname: String) {
        self.path = fullname;
    }

    /// Returns a fullname  of current task.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// `String` - fullname of current task
    pub fn get_current_task(&self) -> &str {
        &self.path
    }

    /// Breaks current loop if exists.
    ///
    /// # Returns
    ///
    /// `Ok(bool)` true if break-signal has been sent
    /// to track state of loop
    pub async fn break_loop(&self) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::BreakLoop(tx))?;
        Ok(rx.await?)
    }

    /// Setting variable value
    ///
    /// # Arguments
    ///
    /// * `key` - Key/Name of variable
    /// * `value` - Variable value
    /// * `warn` - Log warning if variable already exists. Variable still will be
    /// overwritten
    ///
    /// # Returns
    ///
    /// `Ok(bool)` true - if value replaced; false - if not, or `Err(E)` in case
    /// of any channel related error
    async fn setting_var(&self, key: &str, value: AnyValue, warn: bool) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::SetVariable(key.to_owned(), value, warn, tx))?;
        Ok(rx.await?)
    }
}
