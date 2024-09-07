use tokio::sync::oneshot;
use uuid::Uuid;

use crate::inf::{operator, ValueRef};

#[derive(Debug)]
pub enum Demand {
    Set(
        Uuid,
        String,
        ValueRef,
        oneshot::Sender<Result<(), operator::E>>,
    ),
    Get(Uuid, String, oneshot::Sender<Result<ValueRef, operator::E>>),
    Destroy,
}
