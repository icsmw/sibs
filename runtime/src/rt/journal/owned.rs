use crate::*;

#[derive(Debug, Clone)]
pub struct Journal {
    owner: Uuid,
    journal: RtJournal,
}

impl Journal {
    pub fn new(owner: Uuid, journal: RtJournal) -> Self {
        Self { owner, journal }
    }
    pub fn stdout<S: Into<String>>(&self, msg: S) {
        self.journal.stdout(self.owner, msg);
    }

    pub fn stderr<S: Into<String>>(&self, msg: S) {
        self.journal.stderr(self.owner, msg);
    }

    pub fn info<S: Into<String>>(&self, msg: S) {
        self.journal.info(self.owner, msg);
    }

    pub fn debug<S: Into<String>>(&self, msg: S) {
        self.journal.debug(self.owner, msg);
    }

    pub fn err<S: Into<String>>(&self, msg: S) {
        self.journal.err(self.owner, msg);
    }

    pub fn warn<S: Into<String>>(&self, msg: S) {
        self.journal.warn(self.owner, msg);
    }
}
