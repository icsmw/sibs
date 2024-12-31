use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    GetTargetComponent(oneshot::Sender<String>),
    GetTaskParams(oneshot::Sender<(String, Vec<String>)>),
    Destroy(oneshot::Sender<()>),
}
