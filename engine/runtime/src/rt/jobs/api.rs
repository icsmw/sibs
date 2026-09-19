use super::state::JobState;
use crate::*;
#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Create(String, Option<Uuid>, oneshot::Sender<Result<Job, E>>),
    Update(Uuid, JobState, oneshot::Sender<Result<(), E>>),
    Destroy(oneshot::Sender<()>),
}
