use std::{
    path::{Path, PathBuf},
    process::ExitStatus,
};

use crate::inf::term;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SpawnStatus {
    pub command: String,
    pub cwd: PathBuf,
    pub code: Option<i32>,
    pub success: bool,
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
    pub error: Option<String>,
    pub cancelled: bool,
}

impl SpawnStatus {
    pub fn from_res<S: AsRef<str>, P: AsRef<Path>>(
        command: S,
        cwd: P,
        res: Result<ExitStatus, String>,
    ) -> Self {
        match res {
            Ok(status) => Self::from_status(command, cwd, status),
            Err(err) => Self {
                cwd: cwd.as_ref().to_path_buf(),
                command: command.as_ref().to_string(),
                code: None,
                error: Some(err),
                ..Default::default()
            },
        }
    }
    pub fn from_status<S: AsRef<str>, P: AsRef<Path>>(
        command: S,
        cwd: P,
        status: ExitStatus,
    ) -> Self {
        Self {
            cwd: cwd.as_ref().to_path_buf(),
            command: command.as_ref().to_string(),
            code: status.code(),
            success: status.success(),
            ..Default::default()
        }
    }
    pub fn stdout(mut self, stdout: Vec<String>) -> Self {
        self.stdout = stdout;
        self
    }
    pub fn stderr(mut self, stderr: Vec<String>) -> Self {
        self.stderr = stderr;
        self
    }
    pub fn report(&self) -> String {
        format!(
            "{}{}\n{}{}",
            term::styled(format!(
                "[b]command: [/b] [>>] {}\n[b]cwd: [/b] [>>] {}\n[b]STDOUT[/b]:\n",
                self.command,
                self.cwd.display(),
            )),
            self.stdout.join("\n"),
            term::styled("[b]STDERR[/b]:\n"),
            self.stderr.join("\n")
        )
    }
}
