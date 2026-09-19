use super::*;

#[derive(Debug, Clone)]
pub struct Done<'a> {
    job: &'a Job,
}

impl<'a> Done<'a> {
    pub fn new(job: &'a Job) -> Self {
        Self { job }
    }
}

impl Done<'_> {
    pub async fn success<S: ToString>(&self, msg: Option<S>) -> Result<(), E> {
        self.job
            .update_state(JobState::Success(msg.map(|msg| msg.to_string())))
            .await
    }
    pub async fn failed<S: ToString>(&self, msg: Option<S>) -> Result<(), E> {
        self.job
            .update_state(JobState::Failed(msg.map(|msg| msg.to_string())))
            .await
    }
}
