mod api;
mod context;
mod jobs;
mod journal;
mod progressor;

pub use context::*;
pub use jobs::*;
pub use journal::*;
pub use progressor::*;

use crate::*;
use api::*;

use std::{future::Future, pin::Pin};

pub type RtPinnedResult<'a, E> = Pin<Box<dyn Future<Output = RtResult<E>> + 'a + Send>>;
pub type RtResult<E> = Result<RtValue, E>;

#[derive(Debug, Clone)]
pub struct Runtime {
    pub tys: Arc<TypesTable>,
    pub fns: Arc<Fns>,
    pub tasks: Arc<Tasks>,
    tx: UnboundedSender<Demand>,
}

impl Runtime {
    #[tracing::instrument]
    pub fn new(params: RtParameters, tys: TypesTable, fns: Fns, tasks: Tasks) -> Result<Self, E> {
        let (tx, mut rx) = unbounded_channel();
        let inst = Self {
            tx,
            tys: Arc::new(tys),
            fns: Arc::new(fns),
            tasks: Arc::new(tasks),
        };
        let cx = RtContext::new(params.cwd.clone());
        let progress = RtProgress::new()?;
        let journal = RtJournal::new()?;
        spawn(async move {
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::GetRtParameters(tx) => {
                        chk_send_err!(tx.send(params.clone()), DemandId::GetRtParameters);
                    }
                    Demand::CreateOwnedContext(owner, tx) => {
                        let progress = match progress.create_job("job", None).await {
                            Ok(progress) => progress,
                            Err(err) => {
                                chk_send_err!(tx.send(Err(err)), DemandId::CreateOwnedContext);
                                continue;
                            }
                        };
                        chk_send_err!(
                            tx.send(Ok(cx.create_owned(owner, journal.owned(owner), progress))),
                            DemandId::CreateOwnedContext
                        );
                    }
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        chk_err!(cx.destroy().await);
                        chk_err!(progress.destroy().await);
                        chk_err!(journal.destroy().await);
                        chk_send_err!(tx.send(()), DemandId::Destroy);
                        break;
                    }
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        Ok(inst)
    }

    pub async fn get_rt_parameters(&self) -> Result<RtParameters, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::GetRtParameters(tx))?;
        Ok(rx.await?)
    }

    pub async fn create_cx(&self, owner: Uuid) -> Result<Context, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::CreateOwnedContext(owner, tx))?;
        rx.await?
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
