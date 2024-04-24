mod api;
mod error;
mod job;
mod progress;

use crate::inf::{Journal, Level};
use api::*;
use console::style;
pub use error::E;
pub use job::Job;
use progress::*;
use std::{collections::HashMap, fmt};
use tokio::{
    spawn,
    sync::{
        mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub enum OperationResult {
    Success,
    Failed,
}

impl fmt::Display for OperationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OperationResult::Success => style("done").bold().green(),
                OperationResult::Failed => style("fail").bold().red(),
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Tracker {
    tx: UnboundedSender<Demand>,
    state: CancellationToken,
}

impl Tracker {
    pub fn init(journal: Journal) -> Self {
        let (tx, mut rx): (UnboundedSender<Demand>, UnboundedReceiver<Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            state: state.clone(),
        };
        let own = journal.owned(0, String::from("Tracker"));
        let self_ref = instance.clone();
        spawn(async move {
            let mut progress: Progress = Progress::new(journal.cfg.as_ref().clone());
            let mut jobs: HashMap<usize, String> = HashMap::new();
            while let Some(tick) = rx.recv().await {
                match tick {
                    Demand::CreateJob(alias, len, rx) => {
                        let sequence = match progress.create(&alias, len) {
                            Ok(sequence) => sequence,
                            Err(err) => {
                                let _ = own.err_if(
                                    rx.send(Err(E::ProgressBarError(err.to_string())))
                                        .map_err(|_| err),
                                );
                                continue;
                            }
                        };
                        let _ = own.err_if(
                            rx.send(Ok(Job::new(
                                &self_ref,
                                sequence,
                                journal.owned(sequence, alias.clone()),
                            )))
                            .map_err(|_| "Demand::CreateJob"),
                        );
                        jobs.insert(sequence, alias);
                    }
                    Demand::Message(sequence, msg) => {
                        progress.set_message(sequence, &msg);
                    }
                    Demand::Progress(sequence, pos) => {
                        progress.inc(sequence, pos);
                    }
                    Demand::Finished(sequence, result) => {
                        progress.finish(sequence, result.clone());
                        let _ = jobs.remove(&sequence);
                    }
                    Demand::Destroy => {
                        progress.destroy();
                        jobs.iter().for_each(|(seq, alias)| {
                            journal.collecting().close(alias.clone(), *seq, Level::Warn);
                            journal.warn(alias, format!("\"{alias}\" isn't finished"))
                        });
                        break;
                    }
                }
            }
            state.cancel();
        });
        instance
    }

    fn send<T>(result: Result<(), SendError<T>>) {
        if let Err(e) = result {
            panic!("Fail to communicate with tracker: {e}");
        }
    }

    pub async fn create_job(&self, job: &str, max: Option<u64>) -> Result<Job, E> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Demand::CreateJob(job.to_string(), max, tx))
            .map_err(|e| E::ChannelError(format!("Fail to send tick: {e}")))?;
        rx.await?
    }

    pub fn progress(&self, sequence: usize, pos: Option<u64>) {
        Self::send(self.tx.send(Demand::Progress(sequence, pos)));
    }

    pub fn msg(&self, sequence: usize, log: &str) {
        Self::send(self.tx.send(Demand::Message(sequence, log.to_string())));
    }

    pub fn success(&self, sequence: usize) {
        Self::send(
            self.tx
                .send(Demand::Finished(sequence, OperationResult::Success)),
        )
    }

    pub fn fail(&self, sequence: usize) {
        Self::send(
            self.tx
                .send(Demand::Finished(sequence, OperationResult::Failed)),
        );
    }

    pub async fn destroy(&self) -> Result<(), E> {
        self.tx
            .send(Demand::Destroy)
            .map_err(|e| E::ChannelError(format!("Fail to send tick: {e}")))?;
        self.state.cancelled().await;
        Ok(())
    }
}
