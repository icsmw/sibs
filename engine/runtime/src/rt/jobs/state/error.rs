use crate::{rt::jobs::state::JobState, *};
use enum_ids::enum_ids;
use thiserror::Error;

#[derive(Error, Debug)]
#[enum_ids(derive = "Debug")]
pub enum JobStateError {
    #[error("Job {0} already has state {1}")]
    JobStateAlreadySet(Uuid, JobState),
    #[error("Attempt to back job {0} to Panding state")]
    CannotSetPending(Uuid),
    #[error("Switching state of job {0} in invalid order: from {1} to {2}")]
    InvalidOrder(Uuid, JobState, JobState),
    #[error("Attempt to create child of job {0} in state {1}")]
    InvalidState(Uuid, JobState),
    #[error("Cannot finish job {parent} from {current} as {requested}: descendant {child} is still {child_state}")]
    UnfinishedDescendant {
        parent: Uuid,
        current: JobState,
        requested: JobState,
        child: Uuid,
        child_state: JobState,
    },
}
