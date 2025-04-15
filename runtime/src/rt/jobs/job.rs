use std::hash::Hash;

use tokio_util::sync::CancellationToken;

use crate::*;

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

    pub async fn child<S: ToString>(&self, owner: Uuid, alias: S) -> Result<Job, E> {
        self.rt
            .create(owner, alias.to_string(), Some(self.owner))
            .await
    }
}
