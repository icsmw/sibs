use crate::inf::tracker::Tracker;
use std::{collections::HashMap, fmt, time::Instant};

#[derive(Clone, Debug)]
pub enum Level {
    Info,
    Warn,
    Verb,
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
                Self::Verb => "[VERBOSE]",
            }
        )
    }
}

pub trait Logs {
    fn get_tracker(&self) -> &'_ Tracker;

    fn get_alias(&self) -> &'_ str;

    async fn log<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.get_tracker()
            .log(self.get_alias().to_owned(), Level::Info, msg.to_string())
            .await;
    }

    async fn warn<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.get_tracker()
            .log(self.get_alias().to_owned(), Level::Warn, msg.to_string())
            .await;
    }

    async fn verb<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.get_tracker()
            .log(self.get_alias().to_owned(), Level::Verb, msg.to_string())
            .await;
    }

    async fn err<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.get_tracker()
            .log(self.get_alias().to_owned(), Level::Err, msg.to_string())
            .await;
    }
}

#[derive(Clone, Debug)]
pub struct Logger {
    tracker: Tracker,
    alias: String,
}

impl Logger {
    pub fn new(tracker: &Tracker, alias: String) -> Self {
        Self {
            tracker: tracker.clone(),
            alias,
        }
    }
}

impl Logs for Logger {
    fn get_alias(&self) -> &str {
        &self.alias
    }
    fn get_tracker(&self) -> &Tracker {
        &self.tracker
    }
}

pub struct LogMessage {
    alias: String,
    msg: String,
    level: Level,
    time: u128,
}

pub struct Storage {
    messages: Vec<LogMessage>,
    aliases: HashMap<usize, (String, Instant)>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            messages: vec![],
            aliases: HashMap::new(),
        }
    }
    pub fn create_bound(&mut self, sequence: usize, alias: String) {
        self.aliases.insert(sequence, (alias, Instant::now()));
    }
    pub fn add<'a, T>(&mut self, alias: T, msg: T, level: Level)
    where
        T: 'a + ToOwned + ToString,
    {
        self.messages.push(LogMessage {
            alias: alias.to_string(),
            msg: msg.to_string(),
            level,
            time: Instant::now().elapsed().as_millis(),
        });
    }
    pub fn add_bound<'a, T>(&mut self, sequence: &usize, msg: T, level: Level)
    where
        T: 'a + ToOwned + ToString,
    {
        if let Some((alias, _)) = self.aliases.get(sequence) {
            self.messages.push(LogMessage {
                alias: alias.to_string(),
                msg: msg.to_string(),
                level,
                time: Instant::now().elapsed().as_millis(),
            });
        }
    }
    pub fn finish_bound(&mut self, sequence: &usize) {
        if let Some((alias, instant)) = self.aliases.get(sequence) {
            self.messages.push(LogMessage {
                alias: alias.to_string(),
                msg: format!("finished in {}ms", instant.elapsed().as_millis()),
                level: Level::Info,
                time: Instant::now().elapsed().as_millis(),
            });
        }
    }
}
