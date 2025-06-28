use crate::*;
use brec::block;

#[derive(Debug, Default)]
#[block]
pub struct SessionOpenData {
    pub tm: u64,
    pub uuid: [u8; 16],
}

impl SessionOpenData {
    pub fn new(uuid: &Uuid) -> Result<Self, E> {
        Ok(Self {
            tm: Record::tm()?,
            uuid: *uuid.as_bytes(),
        })
    }
}

#[derive(Debug, Default)]
#[block]
pub struct SessionCloseData {
    pub tm: u64,
    pub uuid: [u8; 16],
}

impl SessionCloseData {
    pub fn new(uuid: &Uuid) -> Result<Self, E> {
        Ok(Self {
            tm: Record::tm()?,
            uuid: *uuid.as_bytes(),
        })
    }
}

#[derive(Debug, Default)]
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

impl From<SessionOpenData> for Packet {
    fn from(msg: SessionOpenData) -> Self {
        Packet::new(vec![Block::SessionOpenData(msg)], None)
    }
}

impl From<SessionCloseData> for Packet {
    fn from(msg: SessionCloseData) -> Self {
        Packet::new(vec![Block::SessionCloseData(msg)], None)
    }
}
