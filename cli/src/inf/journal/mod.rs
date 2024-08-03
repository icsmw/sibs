mod api;
mod cfg;
mod error;
mod extentions;
mod owned;
mod report;
mod storage;

use std::fmt::Display;

use api::*;
pub use cfg::*;
pub use error::*;
use extentions::*;
pub use owned::*;
pub use report::*;
pub use storage::*;
use uuid::Uuid;

use std::sync::Arc;
use tokio::{
    spawn,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Journal {
    state: CancellationToken,
    pub cfg: Arc<Configuration>,
    tx: UnboundedSender<Demand>,
}

impl Journal {
    pub fn unwrapped(cfg: Configuration) -> Self {
        Self::init(cfg)
            .map_err(|e| eprintln!("{e}"))
            .expect("Journal has been created")
    }
    pub fn init(cfg: Configuration) -> Result<Self, E> {
        let (tx, mut rx): (UnboundedSender<Demand>, UnboundedReceiver<Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            cfg: Arc::new(cfg.clone()),
            state: state.clone(),
        };
        let mut storage = Storage::new(cfg)?;
        spawn(async move {
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Log(owner, msg, level) => {
                        storage.log(owner, msg, level);
                    }
                    Demand::Report(report) => {
                        storage.report(report);
                    }
                    Demand::Toleranted(uuid) => {
                        storage.add_tolerant(uuid);
                    }
                    Demand::Collect(uuid, msg) => {
                        storage.collect(uuid, msg);
                    }
                    Demand::CollectionClose(owner, uuid, level) => {
                        if let Some(msg) = storage.collected(uuid) {
                            storage.log(owner, msg, level);
                        }
                    }
                    Demand::Flush(tx) => {
                        if storage.flush().is_err() {
                            storage.log("Journal", "Fail to flush log's storage", Level::Err);
                        }
                        if tx.send(()).is_err() {
                            storage.log("Journal", "Fail to responce on Demand::Flush", Level::Err);
                        }
                    }
                    Demand::Destroy => {
                        break;
                    }
                }
            }
            if storage.flush().is_err() {
                storage.log("Journal", "Fail to flush log's storage", Level::Err);
            }
            storage.print();
            state.cancel();
        });
        Ok(instance)
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        let (tx, _rx): (UnboundedSender<Demand>, UnboundedReceiver<Demand>) = unbounded_channel();
        Self {
            tx,
            cfg: Arc::new(Configuration::logs(false)),
            state: CancellationToken::new(),
        }
    }
    pub async fn destroy(&self) -> Result<(), E> {
        self.tx
            .send(Demand::Destroy)
            .map_err(|_e| E::ShutdownFail)?;
        self.state.cancelled().await;
        Ok(())
    }

    pub fn collecting(&self) -> Collecting<'_> {
        Collecting::new(self)
    }

    pub fn report(&self, report: Report) {
        if let Err(_err) = self.tx.send(Demand::Report(report.clone())) {
            eprintln!("Fail to store report; printing instead");
            report.print(false);
        }
    }

    pub fn as_tolerant(&self, uuid: &Uuid) {
        if self.tx.send(Demand::Toleranted(*uuid)).is_err() {
            eprintln!("Fail to mark report/error as tolerant");
        }
    }

    pub async fn flush(&self) {
        use tokio::sync::oneshot;
        let (tx, rx) = oneshot::channel();
        if self.tx.send(Demand::Flush(tx)).is_err() {
            eprintln!("Fail to flush journal");
        }
        if rx.await.is_err() {
            eprintln!("Fail to get response on flushing journal");
        }
    }

    pub fn owned<S: AsRef<str>>(&self, owner: S, uuid: Option<Uuid>) -> OwnedJournal {
        OwnedJournal::new(uuid.unwrap_or_else(Uuid::new_v4), owner, self.clone())
    }

    pub fn info<O, M>(&self, owner: O, msg: M)
    where
        O: AsRef<str>,
        M: AsRef<str>,
    {
        self.insert(owner, msg, Level::Info);
    }

    pub fn debug<O, M>(&self, owner: O, msg: M)
    where
        O: AsRef<str>,
        M: AsRef<str>,
    {
        self.insert(owner, msg, Level::Debug);
    }

    pub fn verb<O, M>(&self, owner: O, msg: M)
    where
        O: AsRef<str>,
        M: AsRef<str>,
    {
        self.insert(owner, msg, Level::Verb);
    }

    pub fn err<O, M>(&self, owner: O, msg: M)
    where
        O: AsRef<str>,
        M: AsRef<str>,
    {
        self.insert(owner, msg, Level::Err);
    }

    pub fn warn<O, M>(&self, owner: O, msg: M)
    where
        O: AsRef<str>,
        M: AsRef<str>,
    {
        self.insert(owner, msg, Level::Warn);
    }

    pub fn err_if<O, T, E>(&self, owner: O, res: Result<T, E>) -> Result<T, E>
    where
        O: AsRef<str>,
        E: Display,
    {
        if let Err(err) = res.as_ref() {
            self.err(owner, err.to_string());
        }
        res
    }

    fn insert<O, M>(&self, owner: O, msg: M, level: Level)
    where
        O: AsRef<str>,
        M: AsRef<str>,
    {
        if let Err(_err) = self.tx.send(Demand::Log(
            owner.as_ref().to_string(),
            msg.as_ref().to_string(),
            level.clone(),
        )) {
            eprintln!("FSL: [{}][{}]: {}", owner.as_ref(), level, msg.as_ref());
        }
    }
}
