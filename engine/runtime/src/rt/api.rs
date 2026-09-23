use tokio_util::sync::CancellationToken;

use crate::*;

pub type DestroyTokenReceiver = oneshot::Receiver<Result<(), E>>;
pub type DestroyTokenSender = oneshot::Sender<Result<(), E>>;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    EmitSignal(String, oneshot::Sender<Result<(), E>>),
    WaitSignal(String, oneshot::Sender<Option<CancellationToken>>),
    WaitersSignal(String, oneshot::Sender<usize>),
    GetRtParameters(oneshot::Sender<RtParameters>),
    CreateInterpreterEnvironment(
        String,
        Option<Uuid>,
        oneshot::Sender<Result<InterpreterEnvironment, E>>,
    ),
    Destroy(oneshot::Sender<Result<DestroyTokenReceiver, E>>),
    Shutdown(DestroyTokenSender, Result<(), E>),
}
