use crate::inf::context::{
    tracker::{Job, E},
    OperationResult,
};
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Demand {
    CreateJob(String, Option<u64>, oneshot::Sender<Result<Job, E>>),
    Progress(usize, Option<u64>),
    Message(usize, String),
    Finished(usize, OperationResult),
    Destroy,
}
