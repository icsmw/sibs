mod api;
mod fns;
mod scopes;
mod tys;

pub use fns::*;
pub use scopes::*;
pub use tys::*;

use crate::*;
use api::*;

#[derive(Debug, Clone)]
pub struct Runtime {
    pub scopes: RtScope,
    pub tys: RtTypes,
    pub fns: RtFns,
    tx: UnboundedSender<Demand>,
}

impl Runtime {
    #[tracing::instrument]
    pub fn new(tys: TypesTable, fns: Fns) -> Self {
        let (tx, mut rx) = unbounded_channel();
        let inst = Self {
            tx,
            scopes: RtScope::new(),
            tys: RtTypes::new(tys),
            fns: RtFns::new(fns),
        };
        let scopes = inst.scopes.clone();
        let tys = inst.tys.clone();
        let fns: RtFns = inst.fns.clone();
        spawn(async move {
            tracing::info!("init demand's listener");
            if let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        chk_err!(scopes.destroy().await);
                        chk_err!(tys.destroy().await);
                        chk_err!(fns.destroy().await);
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
