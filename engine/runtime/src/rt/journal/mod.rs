mod api;
mod owned;
mod record;
pub mod scheme;

use crate::*;
use api::*;
pub use owned::*;
pub use record::*;
use std::{fs, time::Duration};
use tracing::error;

const LOCK_JOURNAL_WAIT_TIMEOUT_MS: u64 = 4000;
const MAX_ATTEMPTS_TO_DROP_STORAGE: u8 = 5;

#[derive(Clone, Debug)]
pub struct RtJournal {
    tx: UnboundedSender<Demand>,
}

impl RtJournal {
    #[tracing::instrument]
    pub fn new(root: &PathBuf) -> Result<Self, E> {
        let (tx, mut rx) = unbounded_channel();
        let instance = Self { tx };
        let journal_path = root.join(SIBS_FOLDER);
        if !journal_path.exists() {
            fs::create_dir_all(&journal_path)?;
        }
        let session = Uuid::new_v4();
        let journal_file = journal_path.join(format!("{session}.brec"));
        let sessions_file = journal_path.join(SESSIONS_FILENAME);
        let mut sessions = get_sessions_storage(&sessions_file)?;
        let mut journal =
            scheme::FileStorage::new(&journal_file, Some(Duration::from_secs(4)), None)?;
        sessions.insert(scheme::SessionOpenData::new(&session)?.into())?;
        spawn(async move {
            tracing::info!("init demand's listener");
            while let Some(demand) = rx.recv().await {
                match demand {
                    Demand::Destroy(tx) => {
                        tracing::info!("got shutdown signal");
                        let _ = scheme::SessionCloseData::new(&session)
                            .map(|msg| {
                                sessions
                                    .insert(msg.into())
                                    .map_err(|err| error!("fail write rec: {err}"))
                            })
                            .map_err(|err| error!("fail to get SessionCloseData: {err}"));
                        chk_send_err!(tx.send(()), DemandId::Destroy);
                        break;
                    }
                    Demand::Write(record) => {
                        let _ = tokio::task::block_in_place(|| {
                            let packet: scheme::Packet = record.as_packet(&session);
                            journal.insert(packet)
                        })
                        .map_err(|err| error!("fail write rec: {err}"));
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

fn get_sessions_storage<'a>(path: &PathBuf) -> Result<scheme::FileStorage<'a>, E> {
    let mut attempts = 0;
    let storage = loop {
        // If mutliple processes running, we might be using same journal file. In this case we should wait
        // until file will be unlocked.
        match scheme::FileStorage::new(
            path,
            Some(Duration::from_millis(LOCK_JOURNAL_WAIT_TIMEOUT_MS)),
            None,
        ) {
            Ok(storage) => break storage,
            Err(err) => match err {
                brec::Error::CrcDismatch
                | brec::Error::DamagedSlot(..)
                | brec::Error::CannotFindFreeSlot => {
                    if attempts >= MAX_ATTEMPTS_TO_DROP_STORAGE {
                        error!(
                            "Fail to open storage with file {} after {attempts}. Please try to remove file manualy and try again",
                            path.to_string_lossy()
                        );
                    }
                    error!(
                        "Storage file {} has been damaged. It will be dropped.",
                        path.to_string_lossy()
                    );
                    fs::remove_file(&path)?;
                    attempts += 1;
                    continue;
                }
                _ => return Err(err.into()),
            },
        }
    };
    Ok(storage)
}
