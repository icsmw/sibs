use super::*;

#[derive(Debug, Clone)]
pub struct Start<'a> {
    job: &'a Job,
}

impl<'a> Start<'a> {
    pub fn new(job: &'a Job) -> Self {
        Self { job }
    }
}

impl Start<'_> {
    pub async fn started<S: ToString>(&self, msg: Option<S>) -> Result<(), E> {
        self.job
            .update_state(JobState::Started(
                msg.map(|msg| msg.to_string()).unwrap_or_default(),
            ))
            .await
    }
}
