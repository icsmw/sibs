mod api;
mod owned;
mod record;

use api::*;
pub use owned::*;
pub use record::*;

use crate::*;

#[derive(Clone, Debug)]
pub struct RtJournal {
    tx: UnboundedSender<Demand>,
}

impl RtJournal {
    #[tracing::instrument]
    pub fn new() -> Result<Self, E> {
        let (tx, mut rx) = unbounded_channel();
        let instance = Self { tx };
        spawn(async move {
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        chk_send_err!(tx.send(()), DemandId::Destroy);
                        break;
                    }
                    Demand::Write(..) => {}
                }
            }
            tracing::info!("shutdown demand's listener");
        });
        Ok(instance)
    }

    pub async fn destroy(&self) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Destroy(tx))?;
        Ok(rx.await?)
    }

    pub(crate) fn create(&self, owner: Uuid, parent: Option<Uuid>) -> Journal {
        Journal::new(owner, parent, self.clone())
    }

    pub fn stdout<S: Into<String>>(&self, owner: Uuid, msg: S) {
        send(&self.tx, Record::stdout(owner, msg));
    }

    pub fn stderr<S: Into<String>>(&self, owner: Uuid, msg: S) {
        send(&self.tx, Record::stderr(owner, msg));
    }

    pub fn info<S: Into<String>>(&self, owner: Uuid, msg: S) {
        send(&self.tx, Record::info(owner, msg));
    }

    pub fn debug<S: Into<String>>(&self, owner: Uuid, msg: S) {
        send(&self.tx, Record::debug(owner, msg));
    }

    pub fn err<S: Into<String>>(&self, owner: Uuid, msg: S) {
        send(&self.tx, Record::err(owner, msg));
    }

    pub fn warn<S: Into<String>>(&self, owner: Uuid, msg: S) {
        send(&self.tx, Record::warn(owner, msg));
    }
}

fn send(tx: &UnboundedSender<Demand>, msg: Result<Record, E>) {
    match msg {
        Ok(msg) => {
            if tx.send(Demand::Write(msg)).is_err() {
                tracing::error!("Fail write message to journal due channel issue");
            }
        }
        Err(err) => {
            tracing::error!("Fail get record for journal: {err}");
        }
    }
}
