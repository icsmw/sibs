use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    Execute(
        Uuid,
        Runtime,
        Vec<FnArgValue>,
        oneshot::Sender<Result<RtValue, LinkedErr<E>>>,
    ),
    Destroy(oneshot::Sender<()>),
}
