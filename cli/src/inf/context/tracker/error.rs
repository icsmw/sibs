use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Channel error: {0}")]
    ChannelError(String),
    #[error("Progress bar error: {0}")]
    ProgressBarError(String),
    #[error("Fail to receive channel message: {0}")]
    RecvError(String),
    #[error("Fail to send channel message: {0}")]
    SendError(String),
}

impl From<oneshot::error::RecvError> for E {
    fn from(value: oneshot::error::RecvError) -> Self {
        E::RecvError(value.to_string())
    }
}
impl<T> From<mpsc::error::SendError<T>> for E {
    fn from(value: mpsc::error::SendError<T>) -> Self {
        E::SendError(value.to_string())
    }
}
