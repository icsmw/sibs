use crate::inf::tracker::Tracker;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Level {
    Info,
    Warn,
    Err,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Info => "",
                Self::Warn => "[WARNING]",
                Self::Err => "[ERROR]",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Logger {
    tracker: Tracker,
    owner: String,
}

impl Logger {
    pub fn new(tracker: &Tracker, owner: String) -> Self {
        Self {
            tracker: tracker.clone(),
            owner,
        }
    }

    pub async fn log<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.tracker
            .log(self.owner.clone(), Level::Info, msg.to_string())
            .await;
    }

    pub async fn warn<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.tracker
            .log(self.owner.clone(), Level::Warn, msg.to_string())
            .await;
    }

    pub async fn err<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.tracker
            .log(self.owner.clone(), Level::Err, msg.to_string())
            .await;
    }
}
