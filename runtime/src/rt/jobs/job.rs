use std::hash::Hash;

use tokio_util::sync::CancellationToken;

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
    }
    pub fn failed<S: ToString>(&self, msg: Option<S>) {
        if let Some(msg) = msg.as_ref() {
            self.job.journal.err(msg.to_string());
        }
        self.job.progress.failed(msg);
    }
}

#[derive(Debug, Clone)]
pub struct Cancel<'a> {
    job: &'a Job,
}

impl Cancel<'_> {
    pub fn success<S: ToString>(&self, msg: Option<S>) {
        if let Some(msg) = msg.as_ref() {
            self.job.journal.debug(msg.to_string());
        }
        self.job.progress.cancelled(msg);
    }
    pub fn failed<S: ToString>(&self, msg: Option<S>) {
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
    pub(crate) parent: Option<Uuid>,
    pub(crate) alias: String,
    pub cancel: CancellationToken,
    rt: RtJobs,
}

impl Job {
    pub fn new<S: ToString>(
        alias: S,
        owner: Uuid,
        parent: Option<Uuid>,
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
            cancel: CancellationToken::new(),
            rt,
        }
    }

    pub fn done(&self) -> Done<'_> {
        Done { job: self }
    }

    pub fn cancel(&self) -> Done<'_> {
        Done { job: self }
    }

    pub async fn child<S: ToString>(&self, owner: Uuid, alias: S) -> Result<Job, E> {
        self.rt
            .create(owner, alias.to_string(), Some(self.owner))
            .await
    }

    pub fn close(&self) {
        self.journal.debug("job is closed");
        // TODO: Change state of progress?
    }
}
