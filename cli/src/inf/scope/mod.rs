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

#[derive(Debug, Clone)]
pub struct Scope {
    tx: UnboundedSender<api::Demand>,
    state: CancellationToken,
}

impl Scope {
    pub fn init(mut cwd: Option<PathBuf>, journal: &Journal) -> Self {
        let (tx, mut rx): (UnboundedSender<api::Demand>, UnboundedReceiver<api::Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
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
                match demand {
                    api::Demand::SetVariable(k, v, warn, tx) => {
                        if warn && vars.contains_key(&k) {
                            journal.warn(format!(
                                "Variable \"{k}\" will be overwritten with new value"
                            ));
                        }
                        let _ = tx.send(vars.insert(k, Arc::new(v)).is_some());
                    }
                    api::Demand::GetVariable(k, tx) => {
                        let _ = tx.send(vars.get(&k).cloned());
                    }
                    api::Demand::GetVariables(tx) => {
                        let _ = tx.send(vars.clone());
                    }
                    api::Demand::SetCwd(path, tx) => {
                        cwd = path;
                        journal.info(format!(
                            "set CWD to: {}",
                            cwd.as_ref()
                                .map(|cwd| cwd.to_string_lossy().to_string())
                                .unwrap_or("no CWD context".to_string())
                        ));
                        let _ = tx.send(());
                    }
                    api::Demand::GetCwd(tx) => {
                        let _ = tx.send(cwd.clone());
                    }
                    Demand::Destroy => {
                        break;
                    }
                }
            }
            state.cancel();
        });
        instance
    }
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
