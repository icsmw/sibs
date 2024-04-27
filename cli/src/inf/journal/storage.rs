use crate::inf::journal::{Configuration, Output, Report, E};
use console::{strip_ansi_codes, Style};
use std::{
    collections::{HashMap, HashSet},
    env::temp_dir,
    fmt,
    fs::{File, OpenOptions},
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum Level {
    Warn,
    Verb,
    Err,
    Debug,
    Info,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Warn => "WARNING",
                Self::Err => "ERROR",
                Self::Verb => "VERBOSE",
                Self::Debug => "DEBUG",
                Self::Info => "INFO",
            }
        )
    }
}

#[derive(Debug)]
pub struct LogMessage {
    owner: String,
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
            Level::Debug => Style::new().white().bold().apply_to(self.level.to_string()),
            Level::Info => Style::new().white().bold().apply_to(self.level.to_string()),
            Level::Verb => Style::new()
                .bright()
                .bold()
                .apply_to(self.level.to_string()),
        };
        format!(
            "[{}][{level}][{}]: {}",
            self.time - since,
            self.owner,
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

#[derive(Debug)]
pub struct Storage {
    uuid: Uuid,
    messages: Vec<LogMessage>,
    reports: Vec<Report>,
    tolarant: HashSet<Uuid>,
    since: u128,
    cfg: Configuration,
    collected: HashMap<usize, String>,
    file: Option<File>,
}

impl Storage {
    pub fn print(&self) {
        let mut reported: HashSet<Uuid> = HashSet::new();
        self.reports.iter().for_each(|r| {
            if let Some(uuid) = r.err_uuid() {
                if !reported.contains(&uuid) {
                    r.print(self.tolarant.contains(&uuid));
                    reported.insert(uuid);
                }
            } else {
                r.print(false);
            }
        });
    }
    pub fn new(cfg: Configuration) -> Result<Self, E> {
        let uuid = Uuid::new_v4();
        let path = if let Some(path) = &cfg.log_file {
            path.to_owned()
        } else {
            temp_dir().join(format!("{}-{uuid}.sibs.log", timestamp()))
        };
        let file = if cfg.writing {
            Some(
                OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(&path)?,
            )
        } else {
            None
        };
        let mut instance = Self {
            uuid,
            messages: Vec::new(),
            reports: Vec::new(),
            tolarant: HashSet::new(),
            cfg,
            since: timestamp(),
            collected: HashMap::new(),
            file,
        };
        instance.log(
            "",
            &format!(
                "Session has been started. {}",
                if instance.file.is_some() {
                    format!("Log file: {}", path.to_string_lossy())
                } else {
                    String::from("Logs aren't writing into file.")
                }
            ),
            Level::Info,
        );
        Ok(instance)
    }
    pub fn log<'a, T>(&mut self, owner: T, msg: T, level: Level)
    where
        T: 'a + ToOwned + ToString,
    {
        self.messages.push(LogMessage {
            owner: owner.to_string(),
            msg: msg.to_string(),
            level,
            time: timestamp(),
        });
        if let Some(msg) = self.messages.last() {
            if matches!(self.cfg.output, Output::Logs) {
                println!("{}", msg.to_ascii_string(self.since));
            }
            if let Some(file) = self.file.as_mut() {
                if let Err(err) = writeln!(
                    file,
                    "{}",
                    strip_ansi_codes(&msg.to_ascii_string(self.since))
                ) {
                    eprintln!("Fail to write logs: {err:?}");
                }
            }
        }
    }
    pub fn flush(&mut self) -> Result<(), E> {
        if let Some(file) = self.file.as_mut() {
            file.flush()?;
        }
        Ok(())
    }
    pub fn report(&mut self, report: Report) {
        self.reports.push(report);
    }
    pub fn add_tolerant(&mut self, uuid: Uuid) {
        self.tolarant.insert(uuid);
    }
    pub fn collect(&mut self, id: usize, msg: String) {
        let offset = " ".repeat(4);
        self.collected
            .entry(id)
            .and_modify(|cnt| cnt.push_str(&format!("\n{offset}{msg}",)))
            .or_insert(format!("\n{offset}{msg}",));
    }
    pub fn collected(&mut self, id: usize) -> Option<String> {
        self.collected.remove(&id)
    }
}
