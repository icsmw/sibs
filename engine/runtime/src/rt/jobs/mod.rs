mod api;
mod entry;
mod job;

use crate::*;
use api::*;
use entry::*;
pub use job::*;
use tokio_util::sync::CancellationToken;

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
            let mut root: JobEntry =
                JobEntry::new("root", Uuid::new_v4(), None, CancellationToken::new());
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        chk_send_err!(tx.send(()), DemandId::Destroy);
                        break;
                    }
                    Demand::Create(owner, alias, parent, tx) => {
                        let job = JobEntry::new(&alias, owner, parent, root.cancel.clone());
                        if let Some(parent_uuid) = parent {
                            let Some(parent_entry) = root.find(&parent_uuid) else {
                                chk_send_err!(
                                    tx.send(Err(E::JobDoesNotExist(parent_uuid))),
                                    DemandId::Create
                                );
                                continue;
                            };
                            if let Err(err) = parent_entry.add_child(&job) {
                                chk_send_err!(tx.send(Err(err)), DemandId::Create);
                                continue;
                            }
                        } else if let Err(err) = root.add_child(&job) {
                            chk_send_err!(tx.send(Err(err)), DemandId::Create);
                            continue;
                        }
                        let progress = match progress.create(owner, &alias, parent).await {
                            Ok(progress) => progress,
                            Err(err) => {
                                chk_send_err!(tx.send(Err(err)), DemandId::Create);
                                continue;
                            }
                        };
                        chk_send_err!(
                            tx.send(Ok(job.as_job(
                                journal.create(owner, parent),
                                progress,
                                inner.clone()
                            ))),
                            DemandId::Create
                        );
                    }
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        Ok(instance)
    }

    pub(crate) async fn create<S: ToString>(
        &self,
        owner: Uuid,
        alias: S,
        parent: Option<Uuid>,
    ) -> Result<Job, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Create(owner, alias.to_string(), parent, tx))?;
        rx.await?
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
