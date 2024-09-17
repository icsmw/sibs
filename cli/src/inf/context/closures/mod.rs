mod api;

use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    elements::Closure,
    inf::{
        operator::{self, E},
        Journal,
    },
};
use api::*;
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct Closures {
    tx: UnboundedSender<Demand>,
    state: CancellationToken,
}

impl Closures {
    pub fn init(journal: &Journal) -> Self {
        let (tx, mut rx): (UnboundedSender<Demand>, UnboundedReceiver<Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
        };
        let own = journal.owned(String::from("Variables"), None);
        spawn(async move {
            let mut map: HashMap<Uuid, Closure> = HashMap::new();
            while let Some(tick) = rx.recv().await {
                match tick {
                    Demand::Set(uuid, closure, tx) => {
                        let _ = own.err_if(
                            tx.send(if map.insert(uuid, closure).is_some() {
                                Err(operator::E::ClosureAlreadySaved(uuid))
                            } else {
                                Ok(())
                            })
                            .map_err(|_| "Demand::Set"),
                        );
                    }
                    Demand::Get(uuid, tx) => {
                        let _ = own.err_if(
                            tx.send(
                                map.get(&uuid)
                                    .cloned()
                                    .ok_or(operator::E::FailToFindClosure(uuid)),
                            )
                            .map_err(|_| "Demand::Get"),
                        );
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

    pub async fn set(&self, uuid: Uuid, closure: Closure) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Set(uuid, closure, tx))
            .map_err(|e| E::Channel(format!("Fail to send set command: {e}")))?;
        rx.await?
    }

    pub async fn get(&self, uuid: Uuid) -> Result<Closure, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Get(uuid, tx))
            .map_err(|e| E::Channel(format!("Fail to send get command: {e}")))?;
        rx.await?
    }

    pub async fn destroy(&self) -> Result<(), E> {
        self.tx
            .send(Demand::Destroy)
            .map_err(|e| E::Channel(format!("Fail to send destroy command: {e}")))?;
        self.state.cancelled().await;
        Ok(())
    }
}
