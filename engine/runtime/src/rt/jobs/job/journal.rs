use crate::*;

#[derive(Debug, Clone)]
pub struct JobJournal {
    identity: JobIdentity,
    journal: RtJournal,
}

impl JobJournal {
    pub(super) fn new(identity: JobIdentity, journal: RtJournal) -> Self {
        Self { identity, journal }
    }

    pub fn stdout<S: Into<String>>(&self, msg: S) {
        self.journal.stdout(&self.identity, msg);
    }

    pub fn stderr<S: Into<String>>(&self, msg: S) {
        self.journal.stderr(&self.identity, msg);
    }

    pub fn info<S: Into<String>>(&self, msg: S) {
        self.journal.info(&self.identity, msg);
    }

    pub fn debug<S: Into<String>>(&self, msg: S) {
        self.journal.debug(&self.identity, msg);
    }

    pub fn err<S: Into<String>>(&self, msg: S) {
        self.journal.err(&self.identity, msg);
    }

    pub fn warn<S: Into<String>>(&self, msg: S) {
        self.journal.warn(&self.identity, msg);
    }
}
