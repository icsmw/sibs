use std::{path::PathBuf, process::ExitStatus};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SpawnStatus {
    Success,
    Failed(Option<i32>),
    Cancelled,
}
