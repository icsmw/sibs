use std::mem;

use crate::*;

#[derive(Debug)]
pub struct Record {
    pub ts: u64,
    pub owner: Uuid,
    pub ty: scheme::RecordTy,
    pub msg: String,
}

impl Record {
    pub fn tm() -> Result<u64, E> {
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map_err(|_| E::Timestamp)?
            .as_secs())
    }

    pub fn stdout<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            ty: scheme::RecordTy::Stdout,
            msg: msg.into(),
        })
    }

    pub fn stderr<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            ty: scheme::RecordTy::Stderr,
            msg: msg.into(),
        })
    }

    pub fn info<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            ty: scheme::RecordTy::Info,
            msg: msg.into(),
        })
    }

    pub fn debug<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            ty: scheme::RecordTy::Debug,
            msg: msg.into(),
        })
    }

    pub fn err<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            ty: scheme::RecordTy::Err,
            msg: msg.into(),
        })
    }

    pub fn warn<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            ty: scheme::RecordTy::Warn,
            msg: msg.into(),
        })
    }
    pub fn as_packet(mut self, session: &Uuid) -> scheme::Packet {
        scheme::Packet::new(
            vec![scheme::Block::Signature(scheme::Signature {
                ts: mem::take(&mut self.ts),
                owner: *self.owner.as_bytes(),
                ty: mem::take(&mut self.ty),
                session: *session.as_bytes(),
            })],
            Some(scheme::Payload::String(mem::take(&mut self.msg))),
        )
    }
}
