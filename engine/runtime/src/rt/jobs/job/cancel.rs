use tokio_util::sync::WaitForCancellationFutureOwned;

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
    /// Begin cancellation of this job and signal its descendants.
    /// The lifecycle owner must call this once; repeated transitions are errors.
    pub async fn cancelling(&self) -> Result<(), E> {
        self.job.sensors.cancel();
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
    pub fn cancellation_owned(&self) -> WaitForCancellationFutureOwned {
        self.job.sensors.cancellation_owned()
    }
    pub fn is_cancelled(&self) -> bool {
        self.job.sensors.is_cancelled()
    }
}
