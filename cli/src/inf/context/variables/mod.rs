mod api;
mod map;

use map::*;
use uuid::Uuid;

use crate::inf::{operator::E, Journal, ValueRef};
use api::*;
use tokio::{
    spawn,
    sync::{
        mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct VariablesMeta {
    tx: UnboundedSender<Demand>,
    state: CancellationToken,
}

impl VariablesMeta {
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
            let mut map: VariablesMap = VariablesMap::default();
            while let Some(tick) = rx.recv().await {
                match tick {
                    Demand::Set(owner, name, ty, tx) => {
                        let _ = own.err_if(
                            tx.send(map.set(&owner, name, ty))
                                .map_err(|_| "Demand::Set"),
                        );
                    }
                    Demand::Get(owner, name, tx) => {
                        let _ =
                            own.err_if(tx.send(map.get(&owner, name)).map_err(|_| "Demand::Get"));
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

    fn send<T>(result: Result<(), SendError<T>>) {
        if let Err(e) = result {
            panic!("Fail to communicate with tracker: {e}");
        }
    }

    pub async fn set<S: AsRef<str>>(
        &self,
        owner: &Uuid,
        name: S,
        value: ValueRef,
    ) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Set(
                owner.to_owned(),
                name.as_ref().to_string(),
                value,
                tx,
            ))
            .map_err(|e| E::Channel(format!("Fail to send set command: {e}")))?;
        rx.await?
    }

    pub async fn get<S: AsRef<str>>(&self, owner: &Uuid, name: S) -> Result<ValueRef, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Get(owner.to_owned(), name.as_ref().to_string(), tx))
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
