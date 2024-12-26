mod api;

use crate::*;
use api::*;

#[derive(Debug, Clone)]
pub struct Runtime {
    pub scopes: RtScope,
    tx: UnboundedSender<Demand>,
}

impl Runtime {
    #[tracing::instrument]
    pub fn new() -> Self {
        let (tx, mut rx) = unbounded_channel();
        let inst = Self {
            tx,
            scopes: RtScope::new(),
        };
        let scopes = inst.scopes.clone();
        spawn(async move {
            tracing::info!("init demand's listener");
            if let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        chk_err!(scopes.destroy().await);
                        chk_send_err!(tx.send(()), DemandId::Destroy);
                    }
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        inst
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
