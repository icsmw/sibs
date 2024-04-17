use crate::inf::journal::Journal;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct OwnedJournal {
    owner: String,
    journal: Journal,
}

impl OwnedJournal {
    pub fn new(owner: String, journal: Journal) -> Self {
        Self { owner, journal }
    }

    pub fn info<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.journal.info(&self.owner, &msg.to_string());
    }

    pub fn debug<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.journal.debug(&self.owner, &msg.to_string());
    }

    pub fn verb<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.journal.verb(&self.owner, &msg.to_string());
    }

    pub fn err<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.journal.err(&self.owner, &msg.to_string());
    }

    pub fn warn<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString + Display,
    {
        self.journal.warn(&self.owner, &msg.to_string());
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
