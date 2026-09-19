mod api;
mod entry;
mod identity;
mod job;
mod sensors;
mod state;

use crate::*;
use api::*;
use entry::*;
pub use identity::*;
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
            let mut root: JobEntry =
                JobEntry::new(JobIdentity::new("root", None, JobVisibility::Hidden));
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
                    Demand::Create(alias, parent, visibility, tx) => {
                        let entry = if let Some(parent_uuid) = parent {
                            let Some(parent) = root.find(&parent_uuid) else {
                                chk_send_err!(
                                    tx.send(Err(E::JobDoesNotExist(parent_uuid))),
                                    DemandId::Create
                                );
                                continue;
                            };
                            match parent.child(alias, visibility) {
                                Ok(job) => job,
                                Err(err) => {
                                    chk_send_err!(tx.send(Err(err)), DemandId::Create);
                                    continue;
                                }
                            }
                        } else {
                            match root.child(alias, visibility) {
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
                    Demand::Path(uuid, filter, tx) => {
                        chk_send_err!(tx.send(root.path(uuid, filter.as_ref())), DemandId::Path);
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
        visibility: JobVisibility,
    ) -> Result<Job, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::Create(alias.to_string(), parent, visibility, tx))?;
        rx.await?
    }

    pub(crate) async fn update(&self, uuid: Uuid, state: JobState) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Update(uuid, state, tx))?;
        rx.await?
    }

    /// Returns a filtered root-to-target snapshot. The filter runs in the jobs
    /// actor and must be short and nonblocking. It applies to the target too.
    /// An unknown target is an error; rejecting every element yields an empty path.
    pub async fn path<F>(&self, uuid: Uuid, filter: F) -> Result<Vec<JobElement>, E>
    where
        F: Fn(&JobIdentity) -> bool + Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Path(uuid, Box::new(filter), tx))?;
        rx.await?
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}

#[cfg(test)]
mod path_tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn actor_path_and_explicit_progress_for_hidden_job() {
        let dir = std::env::temp_dir().join(format!("sibs-path-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let jobs = RtJobs::new(&dir).unwrap();
        let parent = jobs
            .create("parent", None, JobVisibility::Visible)
            .await
            .unwrap();
        let child = parent.child("hidden", JobVisibility::Hidden).await.unwrap();
        let target = child.identity().uuid();
        let path = jobs
            .path(target, |identity| {
                matches!(identity.visibility(), JobVisibility::Visible)
            })
            .await
            .unwrap();
        assert_eq!(path, vec![JobElement::from(parent.identity())]);
        assert!(jobs.path(target, |_| false).await.unwrap().is_empty());
        let missing = Uuid::new_v4();
        assert!(
            matches!(jobs.path(missing, |_| true).await, Err(E::JobDoesNotExist(id)) if id == missing)
        );
        let progress = child.progress().await.unwrap();
        progress.msg("explicit hidden progress");
        let path = jobs
            .path(target, move |identity| {
                identity.uuid() == target || matches!(identity.visibility(), JobVisibility::Visible)
            })
            .await
            .unwrap();
        assert_eq!(
            path,
            vec![
                JobElement::from(parent.identity()),
                JobElement::from(child.identity())
            ]
        );
        jobs.destroy().await.unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }
}
