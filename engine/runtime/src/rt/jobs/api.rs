use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Create(Uuid, String, Option<Uuid>, oneshot::Sender<Result<Job, E>>),
    Destroy(oneshot::Sender<()>),
}
