mod api;

use crate::inf::{context::E, Journal};
use api::*;
use std::collections::HashMap;
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;

enum Action {
    Check(bool),
    Break,
}

#[derive(Debug, Clone)]
pub struct Signals {
    tx: UnboundedSender<api::Demand>,
    state: CancellationToken,
}

impl Signals {
    pub fn init(journal: &Journal) -> Self {
        let (tx, mut rx): (UnboundedSender<api::Demand>, UnboundedReceiver<api::Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
        };
        let journal = journal.owned("Signals", None);
        spawn(async move {
            let mut signals: HashMap<String, CancellationToken> = HashMap::new();
            while let Some(demand) = rx.recv().await {
                let requested = demand.to_string();
                let action = match demand {
                    api::Demand::Get(k, tx) => {
                        Action::Check(tx.send(signals.entry(k).or_default().clone()).is_err())
                    }
                    api::Demand::Emit(k, tx) => {
                        let exists = signals.contains_key(&k);
                        let token = signals.entry(k.clone()).or_default();
                        if token.is_cancelled() {
                            journal.warn(format!("Signal \"{k}\" emitted multiple times."));
                        } else {
                            token.cancel();
                        }
                        Action::Check(tx.send(exists).is_err())
                    }
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
                }
            }
            state.cancel();
        });
        instance
    }

    /// Destroy API loop due sending destroy command and waiting for abort confirmation
    pub async fn destroy(&self) -> Result<(), E> {
        self.tx.send(Demand::Destroy)?;
        self.state.cancelled().await;
        Ok(())
    }

    /// Return signal token. If token doesn't exist - creates it
    ///
    /// # Arguments
    ///
    /// * `key` - Name of signal
    ///
    /// # Returns
    ///
    /// `Ok(CancellationToken)` Signal token
    pub async fn get(&self, key: &str) -> Result<CancellationToken, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Get(key.to_owned(), tx))?;
        Ok(rx.await?)
    }

    /// Emit target signal
    ///
    /// # Arguments
    ///
    /// * `key` - Key/Name of variable
    ///
    /// # Returns
    ///
    /// `Ok(bool)` true if signal had listeners; false - if not
    pub async fn emit(&self, key: &str) -> Result<bool, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Emit(key.to_owned(), tx))?;
        Ok(rx.await?)
    }
}
