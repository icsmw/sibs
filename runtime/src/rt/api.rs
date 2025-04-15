use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    GetRtParameters(oneshot::Sender<RtParameters>),
    CreateContext(
        Uuid,
        String,
        Option<Uuid>,
        oneshot::Sender<Result<Context, E>>,
    ),
    Destroy(oneshot::Sender<()>),
}
