mod api;
mod context;
mod events;
mod scopes;

pub use context::*;
pub use events::*;
pub use scopes::*;

use crate::*;
use api::*;

use std::{future::Future, pin::Pin};

pub type RtPinnedResult<'a, E> = Pin<Box<dyn Future<Output = RtResult<E>> + 'a + Send>>;
pub type RtResult<E> = Result<RtValue, E>;

#[derive(Debug, Clone)]
pub struct Runtime {
    pub scopes: RtScope,
    pub tys: Arc<TypesTable>,
    pub fns: Arc<Fns>,
    pub tasks: Arc<Tasks>,
    pub cx: RtContext,
    pub evns: RtEvents,
    tx: UnboundedSender<Demand>,
}

impl Runtime {
    #[tracing::instrument]
    pub fn new(params: RtParameters, tys: TypesTable, fns: Fns, tasks: Tasks) -> Self {
        let (tx, mut rx) = unbounded_channel();
        let inst = Self {
            tx,
            scopes: RtScope::new(),
            tys: Arc::new(tys),
            fns: Arc::new(fns),
            tasks: Arc::new(tasks),
            cx: RtContext::new(params),
            evns: RtEvents::new(),
        };
        let scopes = inst.scopes.clone();
        let cx = inst.cx.clone();
        let evns = inst.evns.clone();
        spawn(async move {
            tracing::info!("init demand's listener");
            if let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        chk_err!(scopes.destroy().await);
                        chk_err!(cx.destroy().await);
                        chk_err!(evns.destroy().await);
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
