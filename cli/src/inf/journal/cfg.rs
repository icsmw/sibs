use std::{fmt, path::PathBuf};

#[derive(Clone, Debug)]
pub enum Output {
    Progress,
    Logs,
    None,
}

impl TryFrom<String> for Output {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == Output::Logs.to_string() {
            Ok(Output::Logs)
        } else if value == Output::Progress.to_string() {
            Ok(Output::Progress)
        } else if value == Output::None.to_string() {
            Ok(Output::None)
        } else {
            Err(format!(
                "Available options: {}",
                [Output::Logs, Output::Progress, Output::None]
                    .map(|v| v.to_string())
                    .join(", ")
            ))
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Output::Progress => "progress",
                Output::Logs => "logs",
                Output::None => "none",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Configuration {
    pub log_file: Option<PathBuf>,
    pub output: Output,
    pub trace: bool,
    pub writing: bool,
}
impl Configuration {
    pub fn logs(writing: bool) -> Self {
        Configuration {
            log_file: None,
            output: Output::Logs,
            trace: true,
            writing,
        }
    }
    pub fn to_file(filepath: PathBuf) -> Self {
        Configuration {
            log_file: Some(filepath),
            output: Output::Logs,
            trace: true,
            writing: true,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            log_file: None,
            output: Output::Progress,
            trace: false,
            writing: true,
        }
    }
}
