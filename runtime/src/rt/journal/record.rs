use crate::*;

#[derive(Debug)]
pub enum RecordContent {
    Stdout(String),
    Stderr(String),
    Debug(String),
    Err(String),
    Warn(String),
    Info(String),
}

#[derive(Debug)]
pub struct Record {
    ts: u64,
    owner: Uuid,
    content: RecordContent,
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
            content: RecordContent::Stdout(msg.into()),
        })
    }

    pub fn stderr<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            content: RecordContent::Stderr(msg.into()),
        })
    }

    pub fn info<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            content: RecordContent::Info(msg.into()),
        })
    }

    pub fn debug<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            content: RecordContent::Debug(msg.into()),
        })
    }

    pub fn err<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            content: RecordContent::Err(msg.into()),
        })
    }

    pub fn warn<S: Into<String>>(owner: Uuid, msg: S) -> Result<Self, E> {
        Ok(Self {
            ts: Self::tm()?,
            owner,
            content: RecordContent::Warn(msg.into()),
        })
    }
}
