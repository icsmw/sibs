use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Write(Uuid, Record),
    Destroy(oneshot::Sender<()>),
}
