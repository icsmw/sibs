use super::*;

#[derive(Debug, Clone)]
pub struct Cancel<'a> {
    job: &'a Job,
}

impl<'a> Cancel<'a> {
    pub fn new(job: &'a Job) -> Self {
        Self { job }
    }
}

impl Cancel<'_> {
    pub async fn cancel(&self) -> Result<(), E> {
        self.job.update_state(JobState::Cancelling).await
    }
    pub async fn cancelled<S: ToString>(&self, msg: Option<S>) -> Result<(), E> {
        self.job
            .update_state(JobState::Cancelled(msg.map(|msg| msg.to_string())))
            .await
    }
    pub fn cancellation(&self) -> WaitForCancellationFuture<'_> {
        self.job.sensors.cancellation()
    }
    pub fn is_cancelled(&self) -> bool {
        self.job.sensors.is_cancelled()
    }
}
