use std::path::Path;

use crate::*;
use brec::{block, payload};

#[derive(Debug)]
pub struct SessionInfo {
    pub md: SessionMetadata,
    pub open: u64,
    pub close: u64,
    pub uuid: Uuid,
}

impl SessionInfo {
    pub fn new(uuid: Uuid, open: u64, md: SessionMetadata) -> Self {
        SessionInfo {
            uuid,
            md,
            open,
            close: 0,
        }
    }
    pub fn set_close_tm(&mut self, close: u64) {
        self.close = close;
    }
}

#[derive(Debug, Default)]
#[block]
pub struct SessionOpenData {
    pub tm: u64,
    pub uuid: [u8; 16],
}

#[payload(bincode)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SessionMetadata {
    pub cwd: PathBuf,
}

#[derive(Debug, Default)]
#[block]
pub struct SessionCloseData {
    pub tm: u64,
    pub uuid: [u8; 16],
}

#[derive(Debug, Default, Clone)]
pub enum RecordTy {
    Stdout,
    Stderr,
    #[default]
    Debug,
    Err,
    Warn,
    Info,
}

impl From<&RecordTy> for u8 {
    fn from(ty: &RecordTy) -> Self {
        match ty {
            RecordTy::Err => 0,
            RecordTy::Warn => 1,
            RecordTy::Debug => 2,
            RecordTy::Info => 3,
            RecordTy::Stdout => 4,
            RecordTy::Stderr => 5,
        }
    }
}

impl From<RecordTy> for u8 {
    fn from(ty: RecordTy) -> Self {
        match ty {
            RecordTy::Err => 0,
            RecordTy::Warn => 1,
            RecordTy::Debug => 2,
            RecordTy::Info => 3,
            RecordTy::Stdout => 4,
            RecordTy::Stderr => 5,
        }
    }
}

impl TryFrom<u8> for RecordTy {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RecordTy::Err),
            1 => Ok(RecordTy::Warn),
            2 => Ok(RecordTy::Debug),
            3 => Ok(RecordTy::Info),
            4 => Ok(RecordTy::Stdout),
            5 => Ok(RecordTy::Stderr),
            _ => Err(format!("{value} isn't valid RecordTy")),
        }
    }
}

#[derive(Debug, Default)]
pub struct Owner(Uuid);

impl From<&Owner> for [u8; 16] {
    fn from(value: &Owner) -> Self {
        *value.0.as_bytes()
    }
}

#[derive(Default)]
#[block]
pub struct Signature {
    pub ts: u64,
    pub owner: [u8; 16],
    pub session: [u8; 16],
    pub ty: RecordTy,
}

brec::generate!();

impl SessionOpenData {
    pub fn new(uuid: &Uuid) -> Result<Self, E> {
        Ok(Self {
            tm: Record::tm()?,
            uuid: *uuid.as_bytes(),
        })
    }
    pub fn packet<P: AsRef<Path>>(uuid: &Uuid, cwd: P) -> Result<Packet, E> {
        Ok(Packet::new(
            vec![Block::SessionOpenData(SessionOpenData::new(uuid)?)],
            Some(Payload::SessionMetadata(SessionMetadata::new(cwd))),
        ))
    }
}

impl SessionCloseData {
    pub fn new(uuid: &Uuid) -> Result<Self, E> {
        Ok(Self {
            tm: Record::tm()?,
            uuid: *uuid.as_bytes(),
        })
    }
    pub fn packet(uuid: &Uuid) -> Result<Packet, E> {
        Ok(Packet::new(
            vec![Block::SessionCloseData(SessionCloseData::new(uuid)?)],
            None,
        ))
    }
}

impl SessionMetadata {
    pub fn new<P: AsRef<Path>>(cwd: P) -> SessionMetadata {
        SessionMetadata {
            cwd: cwd.as_ref().to_path_buf(),
        }
    }
}
