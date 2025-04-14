use tokio_util::sync::CancellationToken;

use crate::*;

pub struct Job {
    pub journal: Journal,
    pub progress: Progress,
    pub owner: Uuid,
    pub cancel: CancellationToken,
}
