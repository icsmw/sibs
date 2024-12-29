use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Execute(
        Uuid,
        Runtime,
        Vec<RtValue>,
        oneshot::Sender<Result<RtValue, E>>,
    ),
    Destroy(oneshot::Sender<()>),
}
