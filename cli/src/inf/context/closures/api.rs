use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{elements::Closure, inf::operator};

#[derive(Debug)]
pub enum Demand {
    Set(Uuid, Closure, oneshot::Sender<Result<(), operator::E>>),
    Get(Uuid, oneshot::Sender<Result<Closure, operator::E>>),
    Destroy,
}
