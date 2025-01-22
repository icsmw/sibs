mod api;
mod job;
mod render;

pub use job::*;

use crate::*;
use api::*;
use render::*;

#[derive(Clone, Debug)]
pub struct JobsProgress {
    tx: UnboundedSender<Demand>,
}

impl JobsProgress {
    #[tracing::instrument]
    pub fn new() -> Self {
        let (tx, mut rx) = unbounded_channel();
        let instance = Self { tx };
        let this = instance.clone();
        spawn(async move {
            let mut jobs: HashMap<usize, JobProgress> = HashMap::new();
            let mut render: ProgressRender = ProgressRender::new();
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::CreateJob(alias, tx) => {
                        let sequence = match render.add_job(&alias, None) {
                            Ok(sequence) => sequence,
                            Err(err) => {
                                chk_send_err!(tx.send(Err(err)), DemandId::CreateJob);
                                continue;
                            }
                        };
                        let job = JobProgress::new(&this, sequence);
                        chk_send_err!(tx.send(Ok(job)), DemandId::CreateJob);
                    }
                    Demand::Message(sequence, msg) => {
                        render.msg(sequence, &msg);
                    }
                    Demand::ProgressLen(sequence, len) => {
                        render.len(sequence, len);
                    }
                    Demand::Progress(sequence, pos) => {
                        render.inc(sequence, pos);
                    }
                    Demand::Finished(sequence, result) => {
                        let Some(job) = jobs.remove(&sequence) else {
                            continue;
                        };
                        render.finish(sequence, result.into_ext(&job.ts));
                    }
                    Demand::Destroy(tx) => {
                        render.destroy();
                        tracing::info!("got shutdown signal");
                        chk_send_err!(tx.send(()), DemandId::Destroy);
                        break;
                    }
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        instance
    }

    pub async fn create_job<S: AsRef<str>>(&self, job: S) -> Result<JobProgress, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::CreateJob(job.as_ref().to_string(), tx))?;
        rx.await?
    }

    pub fn len(&self, sequence: usize, len: u64) {
        chk_send_err!(
            self.tx.send(Demand::ProgressLen(sequence, len)),
            DemandId::ProgressLen
        );
    }

    pub fn progress(&self, sequence: usize, progress: Option<u64>) {
        chk_send_err!(
            self.tx.send(Demand::Progress(sequence, progress)),
            DemandId::Progress
        );
    }

    pub fn msg<S: AsRef<str>>(&self, sequence: usize, log: S) {
        chk_send_err!(
            self.tx
                .send(Demand::Message(sequence, log.as_ref().to_string())),
            DemandId::Message
        );
    }

    pub fn success<S: AsRef<str>>(&self, sequence: usize, msg: Option<S>) {
        chk_send_err!(
            self.tx.send(Demand::Finished(
                sequence,
                JobResult::Success(msg.map(|msg| msg.as_ref().to_string())),
            )),
            DemandId::Finished
        );
    }

    pub fn fail<S: AsRef<str>>(&self, sequence: usize, msg: Option<S>) {
        chk_send_err!(
            self.tx.send(Demand::Finished(
                sequence,
                JobResult::Failed(msg.map(|msg| msg.as_ref().to_string())),
            )),
            DemandId::Finished
        );
    }

    pub fn cancelled(&self, sequence: usize) {
        chk_send_err!(
            self.tx
                .send(Demand::Finished(sequence, JobResult::Cancelled)),
            DemandId::Finished
        );
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }
}
