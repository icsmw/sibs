mod api;
mod context;
mod jobs;
mod journal;
mod progressor;
mod signals;

pub use context::*;
pub use jobs::*;
pub use journal::*;
pub use progressor::*;
pub use signals::*;
use tokio_util::sync::CancellationToken;

use crate::*;
use api::*;

use std::{future::Future, pin::Pin};

pub type RtPinnedResult<'a, E> = Pin<Box<dyn Future<Output = RtResult<E>> + 'a + Send>>;
pub type RtResult<E> = Result<RtValue, E>;

pub struct SignalsGroup<'a> {
    rt: &'a Runtime,
}

impl SignalsGroup<'_> {
    pub async fn emit_signal<S: ToString>(&self, key: S) -> Result<(), E> {
        self.rt.emit_signal(key).await
    }

    pub async fn wait_signal<S: ToString>(&self, key: S) -> Result<Option<CancellationToken>, E> {
        self.rt.wait_signal(key).await
    }

    pub async fn waiters_signal<S: ToString>(&self, key: S) -> Result<usize, E> {
        self.rt.waiters_signal(key).await
    }
}

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
        let jobs = RtJobs::new()?;
        let mut signals = Signals::default();
        spawn(async move {
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::GetRtParameters(tx) => {
                        chk_send_err!(tx.send(params.clone()), DemandId::GetRtParameters);
                    }
                    Demand::CreateContext(owner, alias, parent, tx) => {
                        let job = match jobs.create(owner, alias, parent).await {
                            Ok(job) => job,
                            Err(err) => {
                                chk_send_err!(tx.send(Err(err)), DemandId::CreateContext);
                                continue;
                            }
                        };
                        chk_send_err!(tx.send(Ok(cx.create(owner, job))), DemandId::CreateContext);
                    }
                    Demand::EmitSignal(key, tx) => {
                        chk_send_err!(tx.send(signals.emit(key)), DemandId::EmitSignal);
                    }
                    Demand::WaitSignal(key, tx) => {
                        chk_send_err!(tx.send(signals.wait(key)), DemandId::WaitSignal);
                    }
                    Demand::WaitersSignal(key, tx) => {
                        chk_send_err!(tx.send(signals.waiters(key)), DemandId::WaitersSignal);
                    }
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        chk_err!(cx.destroy().await);
                        chk_err!(jobs.destroy().await);
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

    pub async fn create_cx<S: ToString>(
        &self,
        owner: Uuid,
        alias: S,
        parent: Option<Uuid>,
    ) -> Result<Context, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::CreateContext(owner, alias.to_string(), parent, tx))?;
        rx.await?
    }

    pub fn signals(&self) -> SignalsGroup<'_> {
        SignalsGroup { rt: self }
    }

    pub(crate) async fn emit_signal<S: ToString>(&self, key: S) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::EmitSignal(key.to_string(), tx))?;
        rx.await?
    }

    pub(crate) async fn wait_signal<S: ToString>(
        &self,
        key: S,
    ) -> Result<Option<CancellationToken>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::WaitSignal(key.to_string(), tx))?;
        Ok(rx.await?)
    }

    pub(crate) async fn waiters_signal<S: ToString>(&self, key: S) -> Result<usize, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::WaitersSignal(key.to_string(), tx))?;
        Ok(rx.await?)
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
