mod api;
mod entry;
mod identity;
mod job;
mod sensors;
mod state;

use crate::*;
use api::*;
use entry::*;
pub(crate) use identity::*;
pub use job::*;
pub(crate) use sensors::*;
pub(crate) use state::*;

#[derive(Clone, Debug)]
pub struct RtJobs {
    tx: UnboundedSender<Demand>,
}

impl RtJobs {
    #[tracing::instrument]
    pub fn new(root: &PathBuf) -> Result<Self, E> {
        let (tx, mut rx) = unbounded_channel();
        let instance = Self { tx };
        let progress = RtProgress::new()?;
        let journal = RtJournal::new(root)?;
        let inner = instance.clone();
        spawn(async move {
            tracing::info!("init demand's listener");
            let mut root: JobEntry = JobEntry::new(JobIdentity::new("root", None));
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Destroy(tx) => {
                        // TODO: jobs shutdown
                        tracing::info!("got shutdown signal");
                        chk_send_err!(journal.destroy().await, DemandId::Destroy);
                        chk_send_err!(progress.destroy().await, DemandId::Destroy);
                        chk_send_err!(tx.send(()), DemandId::Destroy);
                        break;
                    }
                    Demand::Create(alias, parent, tx) => {
                        let entry = if let Some(parent_uuid) = parent {
                            let Some(parent) = root.find(&parent_uuid) else {
                                chk_send_err!(
                                    tx.send(Err(E::JobDoesNotExist(parent_uuid))),
                                    DemandId::Create
                                );
                                continue;
                            };
                            match parent.child(alias) {
                                Ok(job) => job,
                                Err(err) => {
                                    chk_send_err!(tx.send(Err(err)), DemandId::Create);
                                    continue;
                                }
                            }
                        } else {
                            match root.child(alias) {
                                Ok(job) => job,
                                Err(err) => {
                                    chk_send_err!(tx.send(Err(err)), DemandId::Create);
                                    continue;
                                }
                            }
                        };
                        let job = entry.job(inner.clone(), journal.clone(), progress.clone());
                        chk_send_err!(tx.send(Ok(job)), DemandId::Create);
                    }
                    Demand::Update(uuid, state, tx) => {
                        let Some(job) = root.find(&uuid) else {
                            chk_send_err!(tx.send(Err(E::JobDoesNotExist(uuid))), DemandId::Update);
                            continue;
                        };
                        if let Err(err) = job.update(state) {
                            chk_send_err!(tx.send(Err(err)), DemandId::Update);
                            continue;
                        }

                        journal.state(job.identity(), job.state());

                        chk_send_err!(tx.send(Ok(())), DemandId::Create);
                    }
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        Ok(instance)
    }

    pub(crate) async fn create<S: ToString>(
        &self,
        alias: S,
        parent: Option<Uuid>,
    ) -> Result<Job, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Create(alias.to_string(), parent, tx))?;
        rx.await?
    }

    pub(crate) async fn update(&self, uuid: Uuid, state: JobState) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Update(uuid, state, tx))?;
        rx.await?
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
