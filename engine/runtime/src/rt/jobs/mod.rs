mod api;
mod entry;
mod job;
mod state;

use crate::*;
use api::*;
pub(crate) use entry::*;
pub use entry::{JobElement, JobIdentity, JobVisibility};
pub use job::*;
pub(crate) use state::*;

use tokio::{sync::Notify, time::Duration};

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone, Debug)]
pub struct RtJobs {
    tx: UnboundedSender<Demand>,
}

impl RtJobs {
    #[tracing::instrument]
    pub fn new(root: &PathBuf) -> Result<Self, E> {
        let (tx, mut rx) = unbounded_channel();
        let mut master_tx = Some(tx.clone());
        let instance = Self { tx };
        let progress = RtProgress::new()?;
        let journal = RtJournal::new(root)?;
        let inner = instance.clone();
        spawn(async move {
            tracing::info!("init demand's listener");
            let state_listener = Arc::new(Notify::new());
            let mut root: JobEntry = JobEntry::new(
                JobIdentity::new("root", None, JobVisibility::Hidden),
                state_listener.clone(),
            );
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Destroy(tx) => {
                        let Some(master_tx) = master_tx.take() else {
                            chk_send_err!(tx.send(Err(E::JobsShutdowning)), DemandId::Destroy);
                            continue;
                        };
                        if root.is_locked() {
                            chk_send_err!(tx.send(Err(E::JobsShutdowning)), DemandId::Destroy);
                            continue;
                        }
                        tracing::info!("got jobs shutdown signal");
                        if let Err(err) = root.lock() {
                            chk_send_err!(tx.send(Err(err)), DemandId::Destroy);
                            continue;
                        }
                        let snapshot = root.snapshot();
                        let (done_tx, done_rx) = oneshot::channel();
                        let timeout = tokio::time::Instant::now() + SHUTDOWN_TIMEOUT;
                        let state_listener_inner = state_listener.clone();
                        spawn(async move {
                            tracing::info!("jobs shutdown has been started");
                            let result = match tokio::time::timeout_at(timeout, async {
                                loop {
                                    if snapshot.unfinished().is_empty() {
                                        break;
                                    }
                                    state_listener_inner.notified().await;
                                }
                            })
                            .await
                            {
                                Ok(_) => Ok(()),
                                Err(_) => Err(E::JobsShutdownTimeout(SHUTDOWN_TIMEOUT.as_millis())),
                            };
                            chk_send_err!(
                                master_tx.send(Demand::Shutdown(done_tx, result)),
                                DemandId::Shutdown
                            );
                        });
                        chk_send_err!(tx.send(Ok(done_rx)), DemandId::Destroy);
                    }
                    Demand::Create(alias, parent, visibility, tx) => {
                        if root.is_locked() {
                            chk_send_err!(tx.send(Err(E::JobsShutdowning)), DemandId::Create);
                            continue;
                        }
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

                        chk_send_err!(tx.send(Ok(())), DemandId::Update);
                    }
                    Demand::Shutdown(done_tx, results) => {
                        chk_send_err!(journal.destroy().await, DemandId::Shutdown);
                        chk_send_err!(progress.destroy().await, DemandId::Shutdown);
                        chk_send_err!(done_tx.send(results), DemandId::Shutdown);
                        tracing::info!("jobs shutdown has been done");
                        break;
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
        let token = rx.await.map_err(|_| E::RecvError)??;
        token.await.map_err(|_| E::RecvError)?
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
        child.cancel().cancelling().await.unwrap();
        child.cancel().cancelled::<String>(None).await.unwrap();
        parent.cancel().cancelling().await.unwrap();
        parent.cancel().cancelled::<String>(None).await.unwrap();
        jobs.destroy().await.unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }
}

#[cfg(test)]
mod lifecycle_tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn rejected_completion_is_returned_without_a_journal_event() {
        let dir = std::env::temp_dir().join(format!("sibs-lifecycle-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let jobs = RtJobs::new(&dir).unwrap();
        let parent = jobs
            .create("parent", None, JobVisibility::Hidden)
            .await
            .unwrap();
        parent.start().started::<String>(None).await.unwrap();
        let child = parent.child("child", JobVisibility::Hidden).await.unwrap();
        let outcome = JobState::Failed(Some("original execution error".into()));
        let err = jobs
            .update(parent.identity().uuid(), outcome.clone())
            .await
            .unwrap_err();
        assert!(
            matches!(err, E::JobState(JobStateError::UnfinishedDescendant {
            parent: id, requested, child: child_id, child_state: JobState::Created, ..
        }) if id == parent.identity().uuid() && child_id == child.identity().uuid() && requested == outcome)
        );
        // The actor remains alive, the child is untouched, and the parent can retry.
        child.start().started::<String>(None).await.unwrap();
        child.done().failed::<String>(None).await.unwrap();
        parent.done().success::<String>(None).await.unwrap();
        jobs.destroy().await.unwrap();
        let mut reader = JournalReader::new(&dir).unwrap();
        let session = *reader.list().keys().next().unwrap();
        let count = reader.open(&session).unwrap().unwrap();
        let records = reader.read(&session, 0, count).unwrap();
        let events: Vec<_> = records
            .iter()
            .filter(|r| r.uuid == parent.identity().uuid())
            .map(|r| r.event.clone())
            .collect();
        assert_eq!(
            events,
            vec![scheme::EventTy::Started, scheme::EventTy::Success]
        );
        drop(reader);
        std::fs::remove_dir_all(dir).unwrap();
    }
}

#[cfg(test)]
mod shutdown_tests {
    use super::*;

    fn directory() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sibs-shutdown-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn empty_tree_shuts_down_without_waiting_for_timeout() {
        let dir = directory();
        let jobs = RtJobs::new(&dir).unwrap();
        tokio::time::timeout(Duration::from_secs(2), async {
            jobs.destroy().await.unwrap();
        })
        .await
        .unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn shutdown_waits_for_terminal_states_and_keeps_servicing_updates() {
        let dir = directory();
        let jobs = RtJobs::new(&dir).unwrap();
        let parent = jobs
            .create("parent", None, JobVisibility::Hidden)
            .await
            .unwrap();
        let child = parent.child("child", JobVisibility::Hidden).await.unwrap();
        parent.start().started::<String>(None).await.unwrap();
        child.start().started::<String>(None).await.unwrap();

        tokio::time::timeout(Duration::from_secs(2), async {
            let shutting_down = jobs.clone();
            let done = tokio::spawn(async move { shutting_down.destroy().await });
            child.cancel().cancellation().await;
            assert!(parent.cancel().is_cancelled());
            assert!(child.cancel().is_cancelled());
            assert!(matches!(
                parent
                    .child("after cancellation", JobVisibility::Hidden)
                    .await,
                Err(E::Cancelled)
            ));
            assert!(matches!(jobs.destroy().await, Err(E::JobsShutdowning)));
            assert!(matches!(
                jobs.create("late", None, JobVisibility::Hidden).await,
                Err(E::JobsShutdowning)
            ));
            assert!(matches!(
                jobs.create(
                    "late child",
                    Some(parent.identity().uuid()),
                    JobVisibility::Hidden
                )
                .await,
                Err(E::JobsShutdowning)
            ));
            assert!(!jobs
                .path(child.identity().uuid(), |_| true)
                .await
                .unwrap()
                .is_empty());

            child.cancel().cancelling().await.unwrap();
            parent.cancel().cancelling().await.unwrap();
            assert!(!done.is_finished());
            child.cancel().cancelled::<String>(None).await.unwrap();
            assert!(!done.is_finished());
            parent.cancel().cancelled::<String>(None).await.unwrap();
            done.await.unwrap().unwrap();
        })
        .await
        .unwrap();

        // The completion receiver must include journal shutdown and its final records.
        let mut reader = JournalReader::new(&dir).unwrap();
        let session = *reader.list().keys().next().unwrap();
        let count = reader.open(&session).unwrap().unwrap();
        let records = reader.read(&session, 0, count).unwrap();
        assert!(records
            .iter()
            .any(|r| r.uuid == parent.identity().uuid() && r.event == scheme::EventTy::Cancelled));
        drop(reader);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[tokio::test]
    async fn child_creation_reports_cancellation_when_shutdown_wins() {
        let dir = directory();
        let jobs = RtJobs::new(&dir).unwrap();
        let parent = jobs
            .create("parent", None, JobVisibility::Hidden)
            .await
            .unwrap();
        parent.start().started::<String>(None).await.unwrap();

        tokio::time::timeout(Duration::from_secs(2), async {
            let (tx, rx) = oneshot::channel();
            // On this single-threaded runtime the actor cannot process Destroy
            // until child() has checked cancellation and queued Create behind it.
            jobs.tx.send(Demand::Destroy(tx)).unwrap();
            assert!(!parent.cancel().is_cancelled());
            let result = parent.child("racing child", JobVisibility::Hidden).await;
            assert!(matches!(result, Err(E::Cancelled)), "{result:?}");

            let done = rx.await.unwrap().unwrap();
            parent.cancel().cancelling().await.unwrap();
            parent.cancel().cancelled::<String>(None).await.unwrap();
            done.await.unwrap().unwrap();
        })
        .await
        .unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[tokio::test]
    async fn child_creation_keeps_the_child_when_creation_wins() {
        let dir = directory();
        let jobs = RtJobs::new(&dir).unwrap();
        let parent = jobs
            .create("parent", None, JobVisibility::Hidden)
            .await
            .unwrap();
        parent.start().started::<String>(None).await.unwrap();

        tokio::time::timeout(Duration::from_secs(2), async {
            // Poll child() first, then queue Destroy before yielding to the actor.
            // The reply contains a real child even though shutdown cancels its token.
            let (child, done) = tokio::join!(biased;
                parent.child("racing child", JobVisibility::Hidden),
                async {
                    let (tx, rx) = oneshot::channel();
                    jobs.tx.send(Demand::Destroy(tx)).unwrap();
                    rx.await.unwrap()
                },
            );
            let child = child.unwrap();
            let done = done.unwrap();
            assert!(child.cancel().is_cancelled());
            child.cancel().cancelling().await.unwrap();
            child.cancel().cancelled::<String>(None).await.unwrap();
            parent.cancel().cancelling().await.unwrap();
            parent.cancel().cancelled::<String>(None).await.unwrap();
            done.await.unwrap().unwrap();
        })
        .await
        .unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn unfinished_job_returns_timeout() {
        let dir = directory();
        let jobs = RtJobs::new(&dir).unwrap();
        let job = jobs
            .create("unfinished", None, JobVisibility::Hidden)
            .await
            .unwrap();
        let result = tokio::time::timeout(SHUTDOWN_TIMEOUT + Duration::from_secs(2), async {
            jobs.destroy().await
        })
        .await
        .unwrap();
        assert!(
            matches!(result, Err(E::JobsShutdownTimeout(ms)) if ms == SHUTDOWN_TIMEOUT.as_millis())
        );
        assert!(job.cancel().is_cancelled());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
