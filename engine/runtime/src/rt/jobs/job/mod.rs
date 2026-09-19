mod cancel;
mod done;
mod journal;
mod progress;
mod start;

use super::*;
use crate::*;
use cancel::*;
use done::*;
pub(crate) use journal::*;
pub(crate) use progress::*;
use start::*;

use tokio_util::sync::WaitForCancellationFuture;

#[derive(Debug, Clone)]
pub struct Job {
    identity: JobIdentity,
    sensors: JobSensonrs,
    journal: RtJournal,
    progress: RtProgress,
    jobs: RtJobs,
}

impl Job {
    pub fn new(
        identity: JobIdentity,
        sensors: JobSensonrs,
        journal: RtJournal,
        progress: RtProgress,
        jobs: RtJobs,
    ) -> Self {
        Self {
            identity,
            sensors,
            journal,
            progress,
            jobs,
        }
    }

    pub fn identity(&self) -> &JobIdentity {
        &self.identity
    }

    pub fn done(&self) -> Done<'_> {
        Done::new(self)
    }

    pub fn cancel(&self) -> Cancel<'_> {
        Cancel::new(self)
    }

    pub fn start(&self) -> Start<'_> {
        Start::new(self)
    }

    pub fn journal(&self) -> JobJournal {
        JobJournal::new(self.identity.clone(), self.journal.clone())
    }

    pub async fn progress(&self) -> Result<JobProgress, E> {
        JobProgress::new(self.identity.clone(), self.progress.clone()).await
    }

    pub async fn child<S: ToString>(&self, alias: S) -> Result<Job, E> {
        self.jobs
            .create(alias.to_string(), Some(self.identity.uuid()))
            .await
    }

    async fn update_state(&self, state: JobState) -> Result<(), E> {
        self.jobs.update(self.identity.uuid(), state).await
    }
}
