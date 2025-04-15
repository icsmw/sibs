use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Create(
        Uuid,
        String,
        Option<Uuid>,
        oneshot::Sender<Result<Progress, E>>,
    ),
    SetState(Uuid, ProgressState),
    SetMsg(Uuid, String),
    Destroy(oneshot::Sender<()>),
}
