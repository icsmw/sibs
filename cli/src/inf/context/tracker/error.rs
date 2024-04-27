use indicatif::style::TemplateError;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Channel error: {0}")]
    Channel(String),
    #[error("Progress bar error: {0}")]
    ProgressBar(String),
    #[error("Fail to receive channel message: {0}")]
    Recv(String),
    #[error("Fail to send channel message: {0}")]
    Send(String),
}

impl From<oneshot::error::RecvError> for E {
    fn from(value: oneshot::error::RecvError) -> Self {
        E::Recv(value.to_string())
    }
}
impl<T> From<mpsc::error::SendError<T>> for E {
    fn from(value: mpsc::error::SendError<T>) -> Self {
        E::Send(value.to_string())
    }
}

impl From<TemplateError> for E {
    fn from(value: TemplateError) -> Self {
        E::ProgressBar(value.to_string())
    }
}
