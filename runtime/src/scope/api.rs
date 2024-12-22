use std::sync::Arc;

use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    SetParentValue(RtValue, oneshot::Sender<()>),
    GetParentValue(oneshot::Sender<Option<Arc<RtValue>>>),
    DropParentValue(oneshot::Sender<()>),
    EnterScope(Uuid, oneshot::Sender<()>),
    LeaveScope(oneshot::Sender<Result<(), E>>),
    SetVariableValue(String, RtValue, oneshot::Sender<Result<(), E>>),
    GetVariableValue(String, oneshot::Sender<Option<Arc<RtValue>>>),
    Destroy,
}
