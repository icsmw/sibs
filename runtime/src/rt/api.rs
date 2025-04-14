use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    GetRtParameters(oneshot::Sender<RtParameters>),
    CreateOwnedContext(Uuid, oneshot::Sender<Result<Context, E>>),
    Destroy(oneshot::Sender<()>),
}
