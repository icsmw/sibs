mod api;
mod record;
mod store;

use api::*;
pub(crate) use record::*;
pub(crate) use store::*;

use crate::*;

#[derive(Clone, Debug)]
pub struct Journal {
    tx: UnboundedSender<Demand>,
}

impl Journal {
    #[tracing::instrument]
    pub fn new() -> Result<Self, E> {
        let (tx, mut rx) = unbounded_channel();
        let instance = Self { tx };
        let this = instance.clone();
        spawn(async move {
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        chk_send_err!(tx.send(()), DemandId::Destroy);
                        break;
                    }
                    Demand::Write(..) => {}
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        Ok(instance)
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
