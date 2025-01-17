use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    OpenLoop(Uuid, oneshot::Sender<Result<(), E>>),
    CloseLoop(oneshot::Sender<Result<(), E>>),
    SetBreakSignal(oneshot::Sender<Result<(), E>>),
    ChkBreakSignal(Uuid, oneshot::Sender<bool>),
    IsBreakInCurrentScope(oneshot::Sender<bool>),
    OpenReturnContext(Uuid, oneshot::Sender<Result<(), E>>),
    CloseReturnContext(oneshot::Sender<Result<(), E>>),
    SetReturnValue(RtValue, oneshot::Sender<Result<(), E>>),
    WithdrawReturnValue(Uuid, oneshot::Sender<Result<Option<RtValue>, E>>),
    IsReturnValueSetInCurrentCx(oneshot::Sender<bool>),
    Destroy(oneshot::Sender<()>),
}
