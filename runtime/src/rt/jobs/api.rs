use crate::*;

#[derive(Debug)]
#[enum_ids::enum_ids(display)]
pub enum Demand {
    CreateJob(String, oneshot::Sender<Result<JobProgress, E>>),
    ProgressLen(usize, u64),
    Progress(usize, Option<u64>),
    Message(usize, String),
    Finished(usize, JobResult),
    Destroy(oneshot::Sender<()>),
}
