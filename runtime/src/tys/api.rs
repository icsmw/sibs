use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Get(Uuid, oneshot::Sender<Option<DataType>>),
    Destroy(oneshot::Sender<()>),
}
