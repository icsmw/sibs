use crate::*;
use console::style;
use std::{fmt, time::Instant};

#[derive(Debug)]
pub enum JobResult {
    Success(Option<String>),
    Failed(Option<String>),
    Cancelled,
}

impl JobResult {
    pub fn into_ext(self, instant: &Instant) -> JobResultExt {
        match self {
            Self::Success(msg) => JobResultExt::Success(msg, instant.elapsed().as_millis()),
            Self::Failed(msg) => JobResultExt::Failed(msg, instant.elapsed().as_millis()),
            Self::Cancelled => JobResultExt::Cancelled(instant.elapsed().as_millis()),
        }
    }
}

#[derive(Debug)]
pub enum JobResultExt {
    Success(Option<String>, u128),
    Failed(Option<String>, u128),
    Cancelled(u128),
}

impl fmt::Display for JobResultExt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JobResultExt::Success(_, ms) => style(format!("done in {ms}ms")).bold().green(),
                JobResultExt::Failed(_, ms) => style(format!("failed in {ms}ms")).bold().red(),
                JobResultExt::Cancelled(ms) =>
                    style(format!("cancelled in {ms}ms")).bold().yellow(),
            }
        )
    }
}

#[derive(Debug)]
pub struct JobProgress {
    tracker: JobsProgress,
    id: usize,
    pub ts: Instant,
}

impl JobProgress {
    pub fn new(tracker: &JobsProgress, id: usize) -> Self {
        Self {
            tracker: tracker.clone(),
            id,
            ts: Instant::now(),
        }
    }

    pub fn progress(&self, pos: Option<u64>) {
        self.tracker.progress(self.id, pos);
    }

    pub fn len(&self, len: u64) {
        self.tracker.len(self.id, len);
    }

    pub fn output<S: AsRef<str>>(&self, log: S) {
        self.tracker.msg(self.id, log);
    }

    pub fn success<S: AsRef<str>>(&self, msg: Option<S>) {
        self.tracker.success(self.id, msg);
    }

    pub fn fail<S: AsRef<str>>(&self, msg: Option<S>) {
        self.tracker.fail(self.id, msg);
    }

    pub fn cancelled(&self) {
        self.tracker.cancelled(self.id);
    }
}
