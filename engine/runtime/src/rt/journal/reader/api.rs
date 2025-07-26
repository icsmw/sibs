use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    List(oneshot::Sender<HashMap<Uuid, scheme::SessionInfo>>),
    Open(Uuid, oneshot::Sender<Result<Option<usize>, E>>),
    Read(Uuid, usize, usize, oneshot::Sender<Option<Vec<Record>>>),
    Close(Uuid, oneshot::Sender<()>),
    Destroy(oneshot::Sender<()>),
}
