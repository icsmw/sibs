use tokio_util::sync::{CancellationToken, WaitForCancellationFuture};

use crate::*;

#[derive(Debug, Clone)]
pub struct Done<'a> {
    job: &'a Job,
}

impl Done<'_> {
    pub fn success<S: ToString>(&self, msg: Option<S>) {
        if let Some(msg) = msg.as_ref() {
            self.job.journal.debug(msg.to_string());
        }
        self.job.progress.success(msg);
        self.job.close();
    }
    pub fn failed<S: ToString>(&self, msg: Option<S>) {
        if let Some(msg) = msg.as_ref() {
            self.job.journal.err(msg.to_string());
        }
        self.job.progress.failed(msg);
        self.job.close();
    }
}

#[derive(Debug, Clone)]
pub struct Cancel<'a> {
    job: &'a Job,
}

impl Cancel<'_> {
    pub fn success<S: ToString>(&self, msg: Option<S>) {
        self.job.cancel.cancel();
        if let Some(msg) = msg.as_ref() {
            self.job.journal.debug(msg.to_string());
        }
        self.job.progress.cancelled(msg);
    }
    pub fn failed<S: ToString>(&self, msg: Option<S>) {
        self.job.cancel.cancel();
        if let Some(msg) = msg.as_ref() {
            self.job
                .journal
                .err(format!("Cancelled with error: {}", msg.to_string()));
        }
        self.job.progress.cancelled(msg);
    }
}

#[derive(Debug, Clone)]
pub struct Job {
    pub journal: Journal,
    pub progress: Progress,
    pub(crate) owner: Uuid,
    #[allow(dead_code)]
    pub(crate) parent: Option<Uuid>,
    #[allow(dead_code)]
    pub(crate) alias: String,
    cancel: CancellationToken,
    rt: RtJobs,
}

impl Job {
    pub fn new<S: ToString>(
        alias: S,
        owner: Uuid,
        parent: Option<Uuid>,
        cancel: CancellationToken,
        journal: Journal,
        progress: Progress,
        rt: RtJobs,
    ) -> Self {
        Self {
            journal,
            progress,
            owner,
            parent,
            alias: alias.to_string(),
            cancel,
            rt,
        }
    }

    pub fn done(&self) -> Done<'_> {
        Done { job: self }
    }

    pub fn cancel(&self) -> Cancel<'_> {
        Cancel { job: self }
    }

    pub fn cancellation(&self) -> WaitForCancellationFuture<'_> {
        self.cancel.cancelled()
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel.is_cancelled()
    }

    pub async fn cancelled(&self) {
        self.cancel.cancelled().await
    }

    pub async fn child<S: ToString>(&self, owner: Uuid, alias: S) -> Result<Job, E> {
        self.rt
            .create(owner, alias.to_string(), Some(self.owner))
            .await
    }

    pub fn close(&self) {
        self.journal.job_close();
        // TODO: Change state of progress?
    }
}
