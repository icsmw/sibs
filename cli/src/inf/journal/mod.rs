mod api;
mod cfg;
mod owned;
mod report;
mod storage;

use crate::error::E;
use std::fmt::Display;

use api::*;
pub use cfg::*;
pub use owned::*;
pub use report::*;
pub use storage::*;

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
    pub fn init(cfg: Configuration) -> Self {
        let (tx, mut rx): (UnboundedSender<Demand>, UnboundedReceiver<Demand>) =
            unbounded_channel();
        let state = CancellationToken::new();
        let instance = Self {
            tx,
            cfg: Arc::new(cfg.clone()),
            state: state.clone(),
        };
        let mut storage = Storage::new(cfg);
        spawn(async move {
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Log(owner, msg, level) => {
                        storage.log(owner, msg, level);
                    }
                    Demand::Report(report) => {
                        storage.report(report);
                    }
                    Demand::Destroy => {
                        break;
                    }
                }
            }
            storage.print();
            state.cancel();
        });
        instance
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        let (tx, _rx): (UnboundedSender<Demand>, UnboundedReceiver<Demand>) = unbounded_channel();
        Self {
            tx,
            cfg: Arc::new(Configuration::logs()),
            state: CancellationToken::new(),
        }
    }
    pub async fn destroy(&self) -> Result<(), E> {
        self.tx.send(Demand::Destroy).map_err(|_e| E {
            sig: String::from("Journal"),
            msg: String::from("Fail to destroy logger because channel error"),
        })?;
        self.state.cancelled().await;
        Ok(())
    }

    pub fn report(&self, report: Report) {
        if let Err(_err) = self.tx.send(Demand::Report(report.clone())) {
            eprintln!("Fail to store report; printing instead");
            report.print();
        }
    }

    pub fn owned(&self, owner: String) -> OwnedJournal {
        OwnedJournal::new(owner, self.clone())
    }

    pub fn info<'a, T>(&self, owner: T, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.insert(owner, msg, Level::Info);
    }

    pub fn debug<'a, T>(&self, owner: T, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.insert(owner, msg, Level::Debug);
    }

    pub fn verb<'a, T>(&self, owner: T, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.insert(owner, msg, Level::Verb);
    }

    pub fn err<'a, T>(&self, owner: T, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.insert(owner, msg, Level::Err);
    }

    pub fn warn<'a, T>(&self, owner: T, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.insert(owner, msg, Level::Warn);
    }

    fn insert<'a, T>(&self, owner: T, msg: T, level: Level)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        if let Err(_err) = self.tx.send(Demand::Log(
            owner.to_string(),
            msg.to_string(),
            level.clone(),
        )) {
            eprintln!("FSL: [{owner}][{}]: {msg}", level);
        }
    }
}
