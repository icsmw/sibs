use tokio_util::sync::{
    CancellationToken, WaitForCancellationFuture, WaitForCancellationFutureOwned,
};

#[derive(Debug, Clone, Default)]
pub struct JobSensonrs {
    cancel: CancellationToken,
}

impl JobSensonrs {
    pub fn child(&self) -> JobSensonrs {
        JobSensonrs {
            cancel: self.cancel.child_token(),
        }
    }
    pub fn cancel(&self) {
        self.cancel.cancel();
    }
    pub fn cancellation(&self) -> WaitForCancellationFuture<'_> {
        self.cancel.cancelled()
    }
    pub fn cancellation_owned(&self) -> WaitForCancellationFutureOwned {
        self.cancel.clone().cancelled_owned()
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancel.is_cancelled()
    }
}
