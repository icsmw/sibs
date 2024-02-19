mod error;
mod logger;
mod progress;
mod task;

use async_channel::{bounded, unbounded, Receiver, Sender};
use console::style;

pub use error::E;
use logger::Storage;
pub use logger::{Logger, Logs};
use progress::Progress;
pub use task::Task;

#[derive(Clone, Debug)]
pub enum OperationResult {
    Success,
    Failed,
}

impl std::fmt::Display for OperationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
pub enum Tick {
    Started(String, Option<u64>, Sender<usize>),
    Progress(usize, Option<u64>),
    Message(usize, String),
    Log(String, logger::Level, String),
    Finished(usize, OperationResult),
    Shutdown(Sender<()>),
}

#[derive(Clone, Debug)]
pub struct Tracker {
    tx: Sender<Tick>,
}

impl Tracker {
    pub fn new() -> Self {
        let (tx, rx): (Sender<Tick>, Receiver<Tick>) = unbounded();
        async_std::task::spawn(Tracker::run(rx));
        Self { tx }
    }

    pub fn create_logger(&self, owner: String) -> Logger {
        Logger::new(self, owner)
    }

    pub async fn create_job(&self, job: &str, max: Option<u64>) -> Result<Task, E> {
        let (tx_response, rx_response) = bounded(1);
        self.tx
            .send(Tick::Started(job.to_string(), max, tx_response))
            .await
            .map_err(|e| E::ChannelError(format!("Fail to send tick: {e}")))?;
        let id = rx_response
            .recv()
            .await
            .map_err(|e| E::ChannelError(e.to_string()))?;
        Ok(Task::new(self, id, job))
    }

    async fn send<T>(sender: async_channel::Send<'_, T>) {
        if let Err(e) = sender.await {
            Self::err(e);
        }
    }

    pub async fn progress(&self, sequence: usize, pos: Option<u64>) {
        Self::send(self.tx.send(Tick::Progress(sequence, pos))).await;
    }

    pub async fn msg(&self, sequence: usize, log: &str) {
        Self::send(self.tx.send(Tick::Message(sequence, log.to_string()))).await;
    }

    pub async fn success(&self, sequence: usize) {
        Self::send(
            self.tx
                .send(Tick::Finished(sequence, OperationResult::Success)),
        )
        .await
    }

    pub async fn fail(&self, sequence: usize) {
        Self::send(
            self.tx
                .send(Tick::Finished(sequence, OperationResult::Failed)),
        )
        .await;
    }

    pub async fn shutdown(&self) -> Result<(), E> {
        let (tx_response, rx_response) = bounded(1);
        self.tx
            .send(Tick::Shutdown(tx_response))
            .await
            .map_err(|e| E::ChannelError(format!("Fail to send tick: {e}")))?;
        rx_response
            .recv()
            .await
            .map_err(|e| E::ChannelError(e.to_string()))
    }

    async fn run(rx: Receiver<Tick>) -> Result<(), E> {
        let mut storage: Storage = Storage::new();
        let mut progress: Progress = Progress::new();
        let err = loop {
            if let Ok(tick) = rx.recv().await {
                match tick {
                    Tick::Started(alias, len, tx_response) => match progress.create(&alias, len) {
                        Ok(sequence) => {
                            storage.add(alias.as_ref(), "started", logger::Level::Info);
                            storage.create_bound(sequence, alias);
                            if let Err(err) = tx_response.send(sequence).await {
                                break Some(format!("Fail to send response: {err}"));
                            }
                        }
                        Err(err) => {
                            break Some(format!("Fail to send response: {err}"));
                        }
                    },
                    Tick::Message(sequence, msg) => {
                        progress.set_message(sequence, &msg);
                        storage.add_bound(&sequence, msg, logger::Level::Info);
                    }
                    Tick::Log(alias, level, msg) => {
                        storage.add(alias, msg, level);
                    }
                    Tick::Progress(sequence, pos) => {
                        progress.inc(sequence, pos);
                    }
                    Tick::Finished(sequence, result) => {
                        progress.finish(sequence, result.clone());
                        storage.finish_bound(&sequence);
                    }
                    Tick::Shutdown(tx_response) => {
                        progress.shutdown();
                        if let Err(err) = tx_response.send(()).await {
                            break Some(format!("Fail to send response: {err}"));
                        }
                        break None;
                    }
                }
            } else {
                break None;
            }
        };
        if let Some(err) = err {
            panic!("{}", err);
        }
        Ok(())
    }

    async fn log(&self, owner: String, level: logger::Level, msg: String) {
        Self::send(self.tx.send(Tick::Log(owner, level, msg))).await
    }

    fn err<E: std::fmt::Display>(e: E) {
        eprintln!("Fail to communicate with tracker: {e}");
    }
}
