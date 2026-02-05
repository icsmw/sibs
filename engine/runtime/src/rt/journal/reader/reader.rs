use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use tracing::warn;

use crate::*;

pub struct JournalReader<'a> {
    root: PathBuf,
    sessions: scheme::Storage<'a, File>,
    journals: HashMap<Uuid, scheme::Storage<'a, File>>,
}

impl<'a> JournalReader<'a> {
    pub fn new(root: &Path) -> Result<JournalReader<'a>, E> {
        let path = root.join(SIBS_FOLDER);
        Ok(JournalReader {
            root: path.clone(),
            sessions: get_storage(&path.join(SESSIONS_FILENAME))?,
            journals: HashMap::new(),
        })
    }

    pub fn list(&mut self) -> HashMap<Uuid, scheme::SessionInfo> {
        let mut list: HashMap<Uuid, scheme::SessionInfo> = HashMap::new();
        self.sessions.iter().for_each(|pkg| match pkg {
            Ok(pkg) => {
                if let Some(block) = pkg.blocks.first() {
                    match block {
                        scheme::Block::SessionOpenData(data) => {
                            if let Some(scheme::Payload::SessionMetadata(md)) = pkg.payload {
                                list.insert(
                                    Uuid::from_bytes(data.uuid),
                                    scheme::SessionInfo::new(
                                        Uuid::from_bytes(data.uuid),
                                        data.tm,
                                        md,
                                    ),
                                );
                            } else {
                                warn!(
                                    "Found opening session data for {}; but no metadata",
                                    Uuid::from_bytes(data.uuid)
                                );
                            }
                        }
                        scheme::Block::SessionCloseData(data) => {
                            if let Some(entry) = list.get_mut(&Uuid::from_bytes(data.uuid)) {
                                entry.set_close_tm(data.tm);
                                if let Some(scheme::Payload::SessionStat(stat)) = pkg.payload {
                                    entry.set_stat(stat);
                                } else {
                                    warn!(
                                        "No session stat found for closing session {}",
                                        Uuid::from_bytes(data.uuid)
                                    );
                                }
                            } else {
                                warn!(
                                    "Found closing session data for {}; but no opening data",
                                    Uuid::from_bytes(data.uuid)
                                )
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                warn!("Fail extract record: {err}");
            }
        });
        list
    }
    pub fn open(&mut self, uuid: &Uuid) -> Result<Option<usize>, E> {
        if let Some(journal) = self.journals.get(uuid) {
            Ok(Some(journal.count()))
        } else {
            let journal = get_storage(&self.root.join(format!("{uuid}.brec")))?;
            let count = journal.count();
            self.journals.insert(*uuid, journal);
            Ok(Some(count))
        }
    }
    pub fn close(&mut self, uuid: &Uuid) {
        let _ = self.journals.remove(uuid);
    }
    pub fn read(&mut self, uuid: &Uuid, from: usize, len: usize) -> Option<Vec<Record>> {
        let journal = self.journals.get_mut(uuid)?;
        let mut records: Vec<Record> = Vec::new();
        journal.range(from, len).for_each(|pkg| match pkg {
            Ok(pkg) => {
                if let Some(record) = Record::from_packet(pkg) {
                    records.push(record);
                }
            }
            Err(err) => {
                warn!("Fail extract record: {err}");
            }
        });
        Some(records)
    }
}

fn get_storage<'a>(path: &PathBuf) -> Result<scheme::Storage<'a, File>, E> {
    if !path.exists() {
        return Err(E::Storage(format!(
            "Storage file {} doesn't exist",
            path.to_string_lossy()
        )));
    }
    let file = OpenOptions::new().read(true).open(path)?;
    Ok(scheme::Storage::new(file)?)
}
