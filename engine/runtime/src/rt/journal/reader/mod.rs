mod api;
mod reader;

use crate::*;
use api::*;
pub use reader::*;
use tracing::warn;

#[derive(Clone, Debug)]
pub struct RtJournalReader {
    tx: UnboundedSender<Demand>,
}

impl RtJournalReader {
    #[tracing::instrument]
    pub fn new(root: &PathBuf) -> Result<Self, E> {
        let mut reader = JournalReader::new(root)?;
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
                    Demand::List(tx) => {
                        chk_send_err!(tx.send(reader.list()), DemandId::List);
                    }
                    Demand::Open(uuid, tx) => {
                        chk_send_err!(tx.send(reader.open(&uuid)), DemandId::Open);
                    }
                    Demand::Close(uuid, tx) => {
                        chk_send_err!(tx.send(reader.close(&uuid)), DemandId::Close);
                    }
                    Demand::Read(uuid, from, len, tx) => {
                        chk_send_err!(tx.send(reader.read(&uuid, from, len)), DemandId::Read);
                    }
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

    pub async fn list(&self) -> Result<HashMap<Uuid, scheme::SessionInfo>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::List(tx))?;
        Ok(rx.await?)
    }

    pub async fn open(&self, uuid: &Uuid) -> Result<Option<usize>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Open(*uuid, tx))?;
        rx.await?
    }

    pub async fn close(&self, uuid: &Uuid) -> Result<(), E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Close(*uuid, tx))?;
        Ok(rx.await?)
    }

    pub async fn read(
        &mut self,
        uuid: &Uuid,
        from: usize,
        len: usize,
    ) -> Result<Option<Vec<Record>>, E> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Demand::Read(*uuid, from, len, tx))?;
        Ok(rx.await?)
    }
}
