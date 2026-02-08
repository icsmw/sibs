use std::{
    fmt::{self, format, Display},
    mem,
};

use chrono::offset;

use crate::*;

#[derive(Debug)]
pub struct Record {
    pub ts: u64,
    pub owner: Uuid,
    pub parent: Option<Uuid>,
    pub ty: scheme::RecordTy,
    pub event: scheme::EventTy,
    pub msg: String,
}

impl Record {
    pub fn tm() -> Result<u64, E> {
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map_err(|_| E::Timestamp)?
            .as_secs())
    }

    pub fn job_open<S: Into<String>>(owner: Uuid, parent: Option<Uuid>, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            parent,
            ty: scheme::RecordTy::Debug,
            event: scheme::EventTy::JobOpened,
            msg: format!("Job created: {}", msg.into()),
        })
    }

    pub fn job_close(owner: Uuid, parent: Option<Uuid>) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            parent,
            ty: scheme::RecordTy::Debug,
            event: scheme::EventTy::JobClosed,
            msg: "Job closed".to_string(),
        })
    }

    pub fn stdout<S: Into<String>>(owner: Uuid, parent: Option<Uuid>, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            parent,
            ty: scheme::RecordTy::Stdout,
            event: scheme::EventTy::Log,
            msg: msg.into(),
        })
    }

    pub fn stderr<S: Into<String>>(owner: Uuid, parent: Option<Uuid>, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            parent,
            ty: scheme::RecordTy::Stderr,
            event: scheme::EventTy::Log,
            msg: msg.into(),
        })
    }

    pub fn info<S: Into<String>>(owner: Uuid, parent: Option<Uuid>, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            parent,
            ty: scheme::RecordTy::Info,
            event: scheme::EventTy::Log,
            msg: msg.into(),
        })
    }

    pub fn debug<S: Into<String>>(owner: Uuid, parent: Option<Uuid>, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            parent,
            ty: scheme::RecordTy::Debug,
            event: scheme::EventTy::Log,
            msg: msg.into(),
        })
    }

    pub fn err<S: Into<String>>(owner: Uuid, parent: Option<Uuid>, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            parent,
            ty: scheme::RecordTy::Err,
            event: scheme::EventTy::Log,
            msg: msg.into(),
        })
    }

    pub fn warn<S: Into<String>>(owner: Uuid, parent: Option<Uuid>, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            parent,
            ty: scheme::RecordTy::Warn,
            event: scheme::EventTy::Log,
            msg: msg.into(),
        })
    }
    pub fn as_packet(mut self, session: &Uuid) -> scheme::Packet {
        scheme::Packet::new(
            vec![scheme::Block::Signature(scheme::Signature {
                ts: mem::take(&mut self.ts),
                owner: *self.owner.as_bytes(),
                parent: self.parent.map_or([0; 16], |p| *p.as_bytes()),
                ty: mem::take(&mut self.ty),
                event: mem::take(&mut self.event),
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
                parent: if sig.parent != [0; 16] {
                    Some(Uuid::from_bytes(sig.parent))
                } else {
                    None
                },
                ty: sig.ty.clone(),
                event: sig.event.clone(),
                msg,
            })
        } else {
            None
        }
    }
    pub fn to_string<S: Display>(
        &self,
        ty: (bool, u8),
        ts: (bool, u8),
        offset: u16,
        marker: S,
    ) -> String {
        let ty_str = if ty.0 {
            self.ty.colored()
        } else {
            String::new()
        };
        let ts_str = if ts.0 {
            self.ts.to_string()
        } else {
            String::new()
        };
        let ty_width = ty.1 as usize;
        let ty_visible_len = if ty.0 { self.ty.to_string().len() } else { 0 };
        let mut ty_pad = ty_str;
        if ty_visible_len < ty_width {
            ty_pad.push_str(&" ".repeat(ty_width - ty_visible_len));
        }
        let ts_pad = format!("{:width$}", ts_str, width = ts.1 as usize);
        format!(
            "[{}][{}]:{marker}{} {}",
            ty_pad,
            ts_pad,
            " ".repeat(offset as usize),
            self.msg
        )
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}][{}]: {}", self.ty, self.ts, self.msg)
    }
}
