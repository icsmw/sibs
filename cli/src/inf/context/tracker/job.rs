use crate::inf::{
    journal::{Level, OwnedJournal},
    tracker::Tracker,
};
use std::{fmt::Display, time::Instant};

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

    #[allow(dead_code)]
    pub fn info<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.journal.info(msg);
    }

    #[allow(dead_code)]
    pub fn debug<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.journal.debug(msg);
    }

    #[allow(dead_code)]
    pub fn verb<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.journal.verb(msg);
    }

    #[allow(dead_code)]
    pub fn err<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.journal.err(msg);
    }

    #[allow(dead_code)]
    pub fn warn<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.journal.warn(msg);
    }
}
