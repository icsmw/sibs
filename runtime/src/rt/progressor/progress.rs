use crate::*;
use std::time::Instant;

#[derive(Debug)]
pub struct Progress {
    progressor: Progressor,
    pub alias: String,
    pub uuid: Uuid,
    pub parent: Option<Uuid>,
    pub ts: Instant,
}

impl Progress {
    pub fn new<S: AsRef<str>>(alias: S, parent: Option<Uuid>, progressor: Progressor) -> Self {
        Self {
            progressor,
            alias: alias.as_ref().to_string(),
            uuid: Uuid::new_v4(),
            parent,
            ts: Instant::now(),
        }
    }

    pub fn msg<S: AsRef<str>>(&mut self, msg: S) {
        self.progressor.set_msg(&self.uuid, msg);
    }

    pub fn progress(&self, done: u64, total: u64) {
        self.progressor
            .set_state(&self.uuid, ProgressState::Progress(None, done, total));
    }

    pub fn success<S: AsRef<str>>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.uuid,
            ProgressState::Success(msg.map(|s| s.as_ref().to_string())),
        );
    }

    pub fn failed<S: AsRef<str>>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.uuid,
            ProgressState::Failed(msg.map(|s| s.as_ref().to_string())),
        );
    }

    pub fn pending<S: AsRef<str>>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.uuid,
            ProgressState::Pending(msg.map(|s| s.as_ref().to_string())),
        );
    }

    pub fn working<S: AsRef<str>>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.uuid,
            ProgressState::Working(msg.map(|s| s.as_ref().to_string())),
        );
    }

    pub fn cancelled<S: AsRef<str>>(&self, msg: Option<S>) {
        self.progressor.set_state(
            &self.uuid,
            ProgressState::Cancelled(msg.map(|s| s.as_ref().to_string())),
        );
    }

    pub async fn child<S: AsRef<str>>(&self, job: S) -> Result<Progress, E> {
        self.progressor.create_job(job, Some(&self.uuid)).await
    }
}
