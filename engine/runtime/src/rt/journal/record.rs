use std::{fmt, mem};

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
    pub fn from_packet(pkg: scheme::Packet) -> Option<Self> {
        if let (Some(scheme::Block::Signature(sig)), Some(scheme::Payload::String(msg))) =
            (pkg.blocks.first(), pkg.payload)
        {
            Some(Record {
                ts: sig.ts,
                owner: Uuid::from_bytes(sig.owner),
                ty: sig.ty.clone(),
                msg,
            })
        } else {
            None
        }
    }
    pub fn to_string(&self, ty: (bool, u8), ts: (bool, u8)) -> String {
        let ty_str = if ty.0 { self.ty.to_string() } else { String::new() };
        let ts_str = if ts.0 { self.ts.to_string() } else { String::new() };
        let ty_pad = format!("{:width$}", ty_str, width = ty.1 as usize);
        let ts_pad = format!("{:width$}", ts_str, width = ts.1 as usize);
        format!("[{}][{}]: {}", ty_pad, ts_pad, self.msg)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}][{}]: {}", self.ty, self.ts, self.msg)
    }
}
