use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Write(Record),
    Destroy(oneshot::Sender<()>),
}
