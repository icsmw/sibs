use uuid::Uuid;

use crate::inf::journal::{Journal, Level};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct OwnedJournal {
    owner: String,
    journal: Journal,
    uuid: Uuid,
}

impl OwnedJournal {
    pub fn new(uuid: Uuid, owner: String, journal: Journal) -> Self {
        Self {
            owner,
            journal,
            uuid,
        }
    }

    pub fn append(&self, msg: &str) {
        self.journal.collecting().append(self.uuid, msg.to_owned());
    }

    pub fn collected(&self, level: Level) {
        self.journal
            .collecting()
            .close(self.owner.clone(), self.uuid, level);
    }

    #[allow(dead_code)]
    pub fn info<T: AsRef<str>>(&self, msg: T) {
        self.journal.info(&self.owner, msg);
    }

    #[allow(dead_code)]
    pub fn debug<T: AsRef<str>>(&self, msg: T) {
        self.journal.debug(&self.owner, msg);
    }

    #[allow(dead_code)]
    pub fn verb<T: AsRef<str>>(&self, msg: T) {
        self.journal.verb(&self.owner, msg);
    }

    #[allow(dead_code)]
    pub fn err<T: AsRef<str>>(&self, msg: T) {
        self.journal.err(&self.owner, msg);
    }

    #[allow(dead_code)]
    pub fn warn<T: AsRef<str>>(&self, msg: T) {
        self.journal.warn(&self.owner, msg);
    }

    pub fn err_if<T, E>(&self, res: Result<T, E>) -> Result<T, E>
    where
        E: Display,
    {
        if let Err(err) = res.as_ref() {
            self.err(err.to_string());
        }
        res
    }
}
