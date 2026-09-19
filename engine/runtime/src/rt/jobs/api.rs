use super::state::JobState;
use crate::*;

pub type IdentityFilter = Box<dyn Fn(&JobIdentity) -> bool + Send>;

#[enum_ids::enum_ids(display)]
pub enum Demand {
    Create(
        String,
        Option<Uuid>,
        JobVisibility,
        oneshot::Sender<Result<Job, E>>,
    ),
    Path(
        Uuid,
        IdentityFilter,
        oneshot::Sender<Result<Vec<JobElement>, E>>,
    ),
    Update(Uuid, JobState, oneshot::Sender<Result<(), E>>),
    Destroy(oneshot::Sender<()>),
}
