use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Register(JobIdentity, oneshot::Sender<Result<(), E>>),
    SetState(Uuid, ProgressState),
    SetMsg(Uuid, String),
    Destroy(oneshot::Sender<()>),
}
