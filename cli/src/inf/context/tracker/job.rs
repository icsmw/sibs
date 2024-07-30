use crate::inf::{
    journal::{Level, OwnedJournal},
    tracker::Tracker,
};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Job {
    tracker: Tracker,
    id: usize,
    ts: Instant,
    journal: OwnedJournal,
}

impl Job {
    pub fn new(tracker: &Tracker, id: usize, journal: OwnedJournal) -> Self {
        Self {
            tracker: tracker.clone(),
            id,
            ts: Instant::now(),
            journal,
        }
    }

    pub fn progress(&self, pos: Option<u64>) {
        self.tracker.progress(self.id, pos);
    }

    pub fn output(&self, log: &str) {
        self.tracker.msg(self.id, log);
        self.journal.append(log);
    }

    pub fn success(&self) {
        self.info(format!("done in {}ms", self.ts.elapsed().as_millis()));
        self.tracker.success(self.id);
        self.journal.collected(Level::Verb);
    }

    pub fn fail(&self) {
        self.journal.collected(Level::Err);
        self.tracker.fail(self.id);
        self.err(format!("failed in {}ms", self.ts.elapsed().as_millis()));
    }

    pub fn cancelled(&self) {
        self.journal.collected(Level::Warn);
        self.tracker.cancelled(self.id);
        self.warn(format!("cancelled in {}ms", self.ts.elapsed().as_millis()));
    }

    #[allow(dead_code)]
    pub fn info<T: AsRef<str>>(&self, msg: T) {
        self.journal.info(msg);
    }

    #[allow(dead_code)]
    pub fn debug<T: AsRef<str>>(&self, msg: T) {
        self.journal.debug(msg);
    }

    #[allow(dead_code)]
    pub fn verb<T: AsRef<str>>(&self, msg: T) {
        self.journal.verb(msg);
    }

    #[allow(dead_code)]
    pub fn err<T: AsRef<str>>(&self, msg: T) {
        self.journal.err(msg);
    }

    #[allow(dead_code)]
    pub fn warn<T: AsRef<str>>(&self, msg: T) {
        self.journal.warn(msg);
    }
}
