use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    SetParentValue(ParentValue, oneshot::Sender<Result<(), E>>),
    WithdrawParentValue(oneshot::Sender<Result<Option<ParentValue>, E>>),
    DropParentValue(oneshot::Sender<Result<(), E>>),
    OpenScope(Uuid, oneshot::Sender<()>),
    CloseScope(oneshot::Sender<Result<(), E>>),
    EnterScope(Uuid, oneshot::Sender<Result<(), E>>),
    LeaveScope(oneshot::Sender<Result<(), E>>),
    SetVariableValue(String, RtValue, oneshot::Sender<Result<(), E>>),
    GetVariableValue(String, oneshot::Sender<Result<Option<Arc<RtValue>>, E>>),
    Destroy(oneshot::Sender<()>),
}
