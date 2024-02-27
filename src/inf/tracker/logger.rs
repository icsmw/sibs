use crate::inf::tracker::{Configuration, Output, Tracker};
use console::Style;
use std::{
    collections::HashMap,
    fmt,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

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
                Self::Info => "INFO",
                Self::Warn => "WARNING",
                Self::Err => "ERROR",
                Self::Verb => "VERBOSE",
            }
        )
    }
}

pub trait Logs {
    fn get_tracker(&self) -> &'_ Tracker;

    fn get_alias(&self) -> &'_ str;

    fn log<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.get_tracker()
            .log(self.get_alias().to_owned(), Level::Info, msg.to_string());
    }

    fn warn<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.get_tracker()
            .log(self.get_alias().to_owned(), Level::Warn, msg.to_string());
    }

    fn verb<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.get_tracker()
            .log(self.get_alias().to_owned(), Level::Verb, msg.to_string());
    }

    fn err<'a, T>(&self, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        self.get_tracker()
            .log(self.get_alias().to_owned(), Level::Err, msg.to_string());
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

impl LogMessage {
    pub fn to_ascii_string(&self, since: u128) -> String {
        let level = match self.level {
            Level::Err => Style::new().red().bold().apply_to(self.level.to_string()),
            Level::Warn => Style::new()
                .yellow()
                .bold()
                .apply_to(self.level.to_string()),
            Level::Info => Style::new().white().bold().apply_to(self.level.to_string()),
            Level::Verb => Style::new()
                .bright()
                .bold()
                .apply_to(self.level.to_string()),
        };
        format!(
            "[{}][{level}][{}]: {}",
            self.time - since,
            self.alias,
            self.msg
        )
    }
}

fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub struct Storage {
    messages: Vec<LogMessage>,
    aliases: HashMap<usize, (String, Instant)>,
    since: u128,
    cfg: Configuration,
}

impl Storage {
    fn print(&self) {
        if let (Some(msg), true) = (
            self.messages.last(),
            matches!(self.cfg.output, Output::Logs),
        ) {
            println!("{}", msg.to_ascii_string(self.since));
        }
    }
    pub fn new(cfg: Configuration) -> Self {
        Self {
            messages: vec![],
            aliases: HashMap::new(),
            cfg,
            since: timestamp(),
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
            time: timestamp(),
        });
        self.print();
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
                time: timestamp(),
            });
            self.print();
        }
    }
    pub fn finish_bound(&mut self, sequence: &usize) {
        if let Some((alias, instant)) = self.aliases.get(sequence) {
            self.messages.push(LogMessage {
                alias: alias.to_string(),
                msg: format!("finished in {}ms", instant.elapsed().as_millis()),
                level: Level::Info,
                time: timestamp(),
            });
            self.print();
        }
    }
}
