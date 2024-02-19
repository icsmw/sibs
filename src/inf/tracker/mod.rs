mod error;
mod logger;
mod progress;
mod task;

use std::{
    fmt::{self, format},
    path::PathBuf,
};

use async_channel::{bounded, unbounded, Receiver, Sender};
use console::style;

pub use error::E;
use logger::Storage;
pub use logger::{Logger, Logs};
use progress::Progress;
pub use task::Task;

#[derive(Clone, Debug)]
pub enum Output {
    Progress,
    Logs,
    None,
}

impl TryFrom<String> for Output {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == Output::Logs.to_string() {
            Ok(Output::Logs)
        } else if value == Output::Progress.to_string() {
            Ok(Output::Progress)
        } else if value == Output::None.to_string() {
            Ok(Output::None)
        } else {
            Err(format!(
                "Available options: {}",
                [Output::Logs, Output::Progress, Output::None]
                    .map(|v| v.to_string())
                    .join(", ")
            ))
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Output::Progress => "progress",
                Output::Logs => "logs",
                Output::None => "none",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Configuration {
    pub log_file: Option<PathBuf>,
    pub output: Output,
    pub trace: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            log_file: None,
            output: Output::Progress,
            trace: false,
        }
    }
}

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
    async fn run(rx: Receiver<Tick>, cfg: Configuration) -> Result<(), E> {
        let mut storage: Storage = Storage::new(cfg.clone());
        let mut progress: Progress = Progress::new(cfg);
        loop {
            if let Ok(tick) = rx.recv().await {
                match tick {
                    Tick::Started(alias, len, tx_response) => match progress.create(&alias, len) {
                        Ok(sequence) => {
                            storage.add(alias.as_ref(), "started", logger::Level::Info);
                            storage.create_bound(sequence, alias);
                            if let Err(err) = tx_response.send(sequence).await {
                                break Err(E::ChannelError(err.to_string()));
                            }
                        }
                        Err(err) => {
                            break Err(err);
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
                            break Err(E::ChannelError(err.to_string()));
                        }
                        break Ok(());
                    }
                }
            } else {
                break Ok(());
            }
        }
    }

    async fn log(&self, owner: String, level: logger::Level, msg: String) {
        Self::send(self.tx.send(Tick::Log(owner, level, msg))).await
    }

    async fn send<T>(sender: async_channel::Send<'_, T>) {
        if let Err(e) = sender.await {
            panic!("Fail to communicate with tracker: {e}");
        }
    }

    pub fn new(cfg: Configuration) -> Self {
        let (tx, rx): (Sender<Tick>, Receiver<Tick>) = unbounded();
        async_std::task::spawn(Tracker::run(rx, cfg));
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
}
