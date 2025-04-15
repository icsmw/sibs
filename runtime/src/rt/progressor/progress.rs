use crate::*;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Progress {
    progressor: RtProgress,
    pub alias: String,
    pub owner: Uuid,
    pub parent: Option<Uuid>,
    pub ts: Instant,
}

impl Progress {
    pub fn new<S: ToString>(
        owner: Uuid,
        alias: S,
        parent: Option<Uuid>,
        progressor: RtProgress,
    ) -> Self {
        Self {
            progressor,
            alias: alias.to_string(),
            owner,
            parent,
            ts: Instant::now(),
        }
    }

    pub fn msg<S: ToString>(&self, msg: S) {
        self.progressor.set_msg(&self.owner, msg);
    }

    pub fn progress(&self, done: u64, total: u64) {
        self.progressor
            .set_state(&self.owner, ProgressState::Progress(None, done, total));
    }

    pub fn success<S: ToString>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.owner,
            ProgressState::Success(msg.map(|s| s.to_string())),
        );
    }

    pub fn failed<S: ToString>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.owner,
            ProgressState::Failed(msg.map(|s| s.to_string())),
        );
    }

    pub fn pending<S: ToString>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.owner,
            ProgressState::Pending(msg.map(|s| s.to_string())),
        );
    }

    pub fn working<S: ToString>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.owner,
            ProgressState::Working(msg.map(|s| s.to_string())),
        );
    }

    pub fn cancelled<S: ToString>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.owner,
            ProgressState::Cancelled(msg.map(|s| s.to_string())),
        );
    }

    pub(crate) async fn child<S: ToString>(&self, job: S) -> Result<Progress, E> {
        self.progressor
            .create(Uuid::new_v4(), job, Some(self.owner))
            .await
    }
}
