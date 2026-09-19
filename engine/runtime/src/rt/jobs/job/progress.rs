use crate::*;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct JobProgress {
    progressor: RtProgress,
    identity: JobIdentity,
    pub ts: Instant,
}

impl JobProgress {
    pub(super) async fn new(
        identity: JobIdentity,
        progressor: RtProgress,
        path: Vec<JobElement>,
    ) -> Result<Self, E> {
        progressor.register(path).await?;
        Ok(Self {
            progressor,
            identity,
            ts: Instant::now(),
        })
    }

    pub fn msg<S: ToString>(&self, msg: S) {
        self.progressor.set_msg(&self.identity, msg);
    }

    pub fn progress(&self, done: u64, total: u64) {
        self.progressor
            .set_state(&self.identity, ProgressState::Progress(None, done, total));
    }

    pub fn success<S: ToString>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.identity,
            ProgressState::Success(msg.map(|s| s.to_string())),
        );
    }

    pub fn failed<S: ToString>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.identity,
            ProgressState::Failed(msg.map(|s| s.to_string())),
        );
    }

    pub fn pending<S: ToString>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.identity,
            ProgressState::Pending(msg.map(|s| s.to_string())),
        );
    }

    pub fn working<S: ToString>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.identity,
            ProgressState::Working(msg.map(|s| s.to_string())),
        );
    }

    pub fn cancelled<S: ToString>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.identity,
            ProgressState::Cancelled(msg.map(|s| s.to_string())),
        );
    }
}
