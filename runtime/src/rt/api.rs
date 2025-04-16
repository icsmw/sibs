use tokio_util::sync::CancellationToken;

use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    EmitSignal(String, oneshot::Sender<Result<(), E>>),
    WaitSignal(String, oneshot::Sender<Option<CancellationToken>>),
    WaitersSignal(String, oneshot::Sender<usize>),
    GetRtParameters(oneshot::Sender<RtParameters>),
    CreateContext(
        Uuid,
        String,
        Option<Uuid>,
        oneshot::Sender<Result<Context, E>>,
    ),
    Destroy(oneshot::Sender<()>),
}
