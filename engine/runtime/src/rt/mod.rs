mod api;
mod context;
mod env;
mod jobs;
mod journal;
mod progressor;
mod signals;

use crate::*;
use api::*;
pub use context::*;
pub use env::*;
pub use jobs::*;
pub use journal::*;
pub use progressor::*;
pub use signals::*;

use std::{future::Future, pin::Pin};
use tokio_util::sync::CancellationToken;

pub type RtPinnedResult<'a, E> = Pin<Box<dyn Future<Output = RtResult<E>> + 'a + Send>>;
pub type RtResult<E> = Result<RtValue, E>;

pub type GtPinnedResult<'a, E> = Pin<Box<dyn Future<Output = GtResult<E>> + 'a + Send>>;
pub type GtResult<E> = Result<bool, E>;

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
        let mut master_tx = Some(tx.clone());
        let inst = Self {
            tx,
            tys: Arc::new(tys),
            fns: Arc::new(fns),
            tasks: Arc::new(tasks),
        };
        let cx = ExecutionContexts::new(&params.cwd);
        let mut jobs = Some(RtJobs::new(&params.cwd)?);
        let mut signals = Signals::default();
        let rt_inner = inst.clone();
        spawn(async move {
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::GetRtParameters(tx) => {
                        chk_send_err!(tx.send(params.clone()), DemandId::GetRtParameters);
                    }
                    Demand::CreateInterpreterEnvironment(alias, parent, tx) => {
                        let Some(jobs) = jobs.as_ref() else {
                            chk_send_err!(tx.send(Err(E::RtShutdowning)), DemandId::Destroy);
                            continue;
                        };
                        let job = match jobs.create(alias, parent, JobVisibility::Hidden).await {
                            Ok(job) => job,
                            Err(err) => {
                                chk_send_err!(
                                    tx.send(Err(err)),
                                    DemandId::CreateInterpreterEnvironment
                                );
                                continue;
                            }
                        };
                        let env = InterpreterEnvironment::new(
                            rt_inner.clone(),
                            cx.create(job.identity().uuid()),
                            job,
                        );
                        chk_send_err!(tx.send(Ok(env)), DemandId::CreateInterpreterEnvironment);
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
                        let (Some(master_tx), Some(jobs)) = (master_tx.take(), jobs.take()) else {
                            chk_send_err!(tx.send(Err(E::RtShutdowning)), DemandId::Destroy);
                            continue;
                        };
                        let (done_tx, done_rx) = oneshot::channel();
                        spawn(async move {
                            tracing::info!("shutdown has been started");
                            chk_send_err!(
                                master_tx.send(Demand::Shutdown(done_tx, jobs.destroy().await)),
                                DemandId::Shutdown
                            );
                        });
                        chk_send_err!(tx.send(Ok(done_rx)), DemandId::Destroy);
                    }
                    Demand::Shutdown(done_tx, results) => {
                        chk_err!(cx.destroy().await);
                        chk_send_err!(done_tx.send(results), DemandId::Shutdown);
                        tracing::info!("shutdown has been done");
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

    pub async fn create_interpreter_env<S: ToString>(
        &self,
        alias: S,
        parent: Option<Uuid>,
    ) -> Result<InterpreterEnvironment, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::CreateInterpreterEnvironment(
            alias.to_string(),
            parent,
            tx,
        ))?;
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
        let token = rx.await.map_err(|_| E::RecvError)??;
        token.await.map_err(|_| E::RecvError)?
    }
}
