use std::{fmt, path::Path};

use crate::*;
use brec::prelude::*;

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
    pub fn set_stat(&mut self, stat: SessionStat) {
        self.md.stat = Some(stat);
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
    pub stat: Option<SessionStat>,
}

#[payload(bincode)]
#[derive(Debug, Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
pub struct SessionStat {
    pub errs: u32,
    pub warns: u32,
    pub infos: u32,
    pub debugs: u32,
    pub stdouts: u32,
    pub stderrs: u32,
    pub events: u32,
}

impl SessionStat {
    pub fn inc(&mut self, ty: &RecordTy) {
        match ty {
            RecordTy::Err => self.errs += 1,
            RecordTy::Warn => self.warns += 1,
            RecordTy::Info => self.infos += 1,
            RecordTy::Debug => self.debugs += 1,
            RecordTy::Stdout => self.stdouts += 1,
            RecordTy::Stderr => self.stderrs += 1,
            RecordTy::Event => self.events += 1,
        }
    }
}

#[derive(Debug, Default)]
#[block]
pub struct SessionCloseData {
    pub tm: u64,
    pub uuid: [u8; 16],
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum RecordTy {
    Event,
    Stdout,
    Stderr,
    #[default]
    Debug,
    Err,
    Warn,
    Info,
}

impl fmt::Display for RecordTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordTy::Event => write!(f, "EVENT"),
            RecordTy::Stdout => write!(f, "STDOUT"),
            RecordTy::Stderr => write!(f, "STDERR"),
            RecordTy::Debug => write!(f, "DEBUG "),
            RecordTy::Err => write!(f, "ERROR "),
            RecordTy::Warn => write!(f, "WARN  "),
            RecordTy::Info => write!(f, "INFO  "),
        }
    }
}

impl RecordTy {
    pub fn colored(&self) -> String {
        match self {
            RecordTy::Event => format!("\x1b[32m{}\x1b[0m", self),
            RecordTy::Stdout => format!("\x1b[32m{}\x1b[0m", self),
            RecordTy::Stderr => format!("\x1b[31m{}\x1b[0m", self),
            RecordTy::Debug => format!("\x1b[34m{}\x1b[0m", self),
            RecordTy::Err => format!("\x1b[31m{}\x1b[0m", self),
            RecordTy::Warn => format!("\x1b[33m{}\x1b[0m", self),
            RecordTy::Info => format!("\x1b[36m{}\x1b[0m", self),
        }
    }
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
            RecordTy::Event => 6,
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
            RecordTy::Event => 6,
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
            6 => Ok(RecordTy::Event),
            _ => Err(format!("{value} isn't valid RecordTy")),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum EventTy {
    #[default]
    Log,
    Created,
    Started,
    Cancelling,
    Cancelled,
    Success,
    Failed,
}

impl From<&EventTy> for u8 {
    fn from(ty: &EventTy) -> Self {
        match ty {
            EventTy::Log => 0,
            EventTy::Created => 1,
            EventTy::Started => 2,
            EventTy::Cancelling => 3,
            EventTy::Cancelled => 4,
            EventTy::Success => 5,
            EventTy::Failed => 6,
        }
    }
}

impl From<EventTy> for u8 {
    fn from(ty: EventTy) -> Self {
        match ty {
            EventTy::Log => 0,
            EventTy::Created => 1,
            EventTy::Started => 2,
            EventTy::Cancelling => 3,
            EventTy::Cancelled => 4,
            EventTy::Success => 5,
            EventTy::Failed => 6,
        }
    }
}

impl TryFrom<u8> for EventTy {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EventTy::Log),
            1 => Ok(EventTy::Created),
            2 => Ok(EventTy::Started),
            3 => Ok(EventTy::Cancelling),
            4 => Ok(EventTy::Cancelled),
            5 => Ok(EventTy::Success),
            6 => Ok(EventTy::Failed),
            _ => Err(format!("{value} isn't valid EventTy")),
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
    pub uuid: [u8; 16],
    pub parent: [u8; 16],
    pub session: [u8; 16],
    pub ty: RecordTy,
    pub event: EventTy,
}

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
    pub fn packet(uuid: &Uuid, stat: SessionStat) -> Result<Packet, E> {
        Ok(Packet::new(
            vec![Block::SessionCloseData(SessionCloseData::new(uuid)?)],
            Some(Payload::SessionStat(stat)),
        ))
    }
}

impl SessionMetadata {
    pub fn new<P: AsRef<Path>>(cwd: P) -> SessionMetadata {
        SessionMetadata {
            cwd: cwd.as_ref().to_path_buf(),
            stat: None,
        }
    }
}

brec::generate!();
