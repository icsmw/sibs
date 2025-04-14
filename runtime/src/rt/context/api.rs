use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum DemandCommand {
    SetParentValue(ParentValue, oneshot::Sender<Result<(), E>>),
    WithdrawParentValue(oneshot::Sender<Result<Option<ParentValue>, E>>),
    DropParentValue(oneshot::Sender<Result<(), E>>),
    OpenScope(Uuid, oneshot::Sender<()>),
    CloseScope(oneshot::Sender<Result<(), E>>),
    EnterScope(Uuid, oneshot::Sender<Result<(), E>>),
    LeaveScope(oneshot::Sender<Result<(), E>>),
    InsertVariable(String, RtValue, oneshot::Sender<Result<(), E>>),
    UpdateVariableValue(String, RtValue, oneshot::Sender<Result<(), E>>),
    GetVariableValue(String, oneshot::Sender<Result<Option<Arc<RtValue>>, E>>),
    OpenLoop(Uuid, oneshot::Sender<Result<(), E>>),
    CloseLoop(oneshot::Sender<Result<(), E>>),
    IsLoopStopped(oneshot::Sender<bool>),
    SetBreakSignal(oneshot::Sender<Result<(), E>>),
    OpenReturnContext(Uuid, oneshot::Sender<Result<(), E>>),
    CloseReturnContext(oneshot::Sender<Result<(), E>>),
    SetReturnValue(RtValue, oneshot::Sender<Result<(), E>>),
    WithdrawReturnValue(Uuid, oneshot::Sender<Result<Option<RtValue>, E>>),
    GetCwd(oneshot::Sender<PathBuf>),
    SetCwd(PathBuf, oneshot::Sender<()>),
}

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Command(Uuid, DemandCommand),
    Destroy(oneshot::Sender<()>),
}
