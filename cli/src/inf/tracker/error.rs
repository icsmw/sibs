use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Channel error: {0}")]
    ChannelError(String),
    #[error("Progress bar error: {0}")]
    ProgressBarError(String),
}
