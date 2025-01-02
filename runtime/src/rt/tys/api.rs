use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Get(Uuid, oneshot::Sender<Option<Ty>>),
    Destroy(oneshot::Sender<()>),
}
