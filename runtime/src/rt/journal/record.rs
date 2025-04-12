use crate::*;

#[derive(Debug)]
pub enum RecordContent {
    Stdout(String),
    Debug(String),
    Err(String),
    Warn(String),
    Info(String),
}

#[derive(Debug)]
pub struct Record {
    ts: u128,
    owner: Uuid,
    content: RecordContent,
}
