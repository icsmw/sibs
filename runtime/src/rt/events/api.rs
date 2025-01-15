use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    OpenLoop(Uuid, oneshot::Sender<Result<(), E>>),
    CloseLoop(oneshot::Sender<Result<(), E>>),
    SetBreakSignal(oneshot::Sender<Result<(), E>>),
    ChkBreakSignal(Uuid, oneshot::Sender<bool>),
    IsBreakInCurrentScope(oneshot::Sender<bool>),
    Destroy(oneshot::Sender<()>),
}
