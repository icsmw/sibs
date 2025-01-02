mod api;

use crate::*;
use api::*;
use tokio::spawn;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RtTypes {
    tx: UnboundedSender<Demand>,
}

impl RtTypes {
    #[tracing::instrument]
    pub fn new(tys: TypesTable) -> Self {
        let (tx, mut rx) = unbounded_channel();
        spawn(async move {
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Get(uuid, tx) => {
                        chk_send_err!({ tx.send(tys.get(&uuid)) }, DemandId::Get);
                    }
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        chk_send_err!(tx.send(()), DemandId::Destroy);
                        break;
                    }
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        Self { tx }
    }

    pub async fn get_ty(&self, uuid: &Uuid) -> Result<Option<Ty>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Get(*uuid, tx))?;
        Ok(rx.await?)
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
