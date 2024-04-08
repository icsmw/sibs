mod error;

use crate::inf::tracker::{Logs, Task};

use std::{
    path::PathBuf,
    process::{ExitStatus, Stdio},
};
use tokio::{
    join,
    process::{Child, Command},
};
use tokio_stream::StreamExt;
use tokio_util::codec::{self, LinesCodec, LinesCodecError};

pub use error::E;

#[cfg(windows)]
fn spawn(command: &str, cwd: &PathBuf) -> Result<Child, E> {
    let (cmd, args) = parse_command(command);
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    Command::new(cmd)
        .args(args)
        .current_dir(cwd)
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| E::Setup(e.to_string()))
}

#[cfg(not(windows))]
fn spawn(command: &str, cwd: &PathBuf) -> Result<Child, E> {
    let (cmd, args) = parse_command(command);
    Command::new(cmd)
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| E::Setup(e.to_string()))
}

fn parse_command(command: &str) -> (&str, Vec<&str>) {
    let mut parts = command.split_ascii_whitespace().collect::<Vec<&str>>();
    (parts.remove(0), parts)
}

pub struct RunResult {
    pub stdout: String,
    pub stdin: String,
    pub status: ExitStatus,
}

pub async fn run(command: &str, cwd: &PathBuf, task: &Task) -> Result<RunResult, E> {
    let mut child = spawn(command, cwd)?;
    let mut stdout = codec::FramedRead::new(
        child
            .stdout
            .take()
            .ok_or_else(|| E::Setup(String::from("Fail to get stdout handle")))?,
        LinesCodec::default(),
    );
    let mut stderr = codec::FramedRead::new(
        child
            .stderr
            .take()
            .ok_or_else(|| E::Setup(String::from("Fail to get stderr handle")))?,
        LinesCodec::default(),
    );
    fn post_logs(line: Result<String, LinesCodecError>, task: &Task) -> String {
        match line {
            Ok(line) => {
                task.msg(line.trim_end());
                task.progress(None);
                line
            }
            Err(err) => {
                task.err(format!("Error during decoding command output: {err}",));
                String::new()
            }
        }
    }
    let (stdout, stdin) = join!(
        async {
            let mut lines = String::new();
            while let Some(line) = stdout.next().await {
                lines = format!("{lines}\n{}", post_logs(line, task));
            }
            lines
        },
        async {
            let mut lines = String::new();
            while let Some(line) = stderr.next().await {
                lines = format!("{lines}\n{}", post_logs(line, task));
            }
            lines
        }
    );
    Ok(RunResult {
        stdin,
        stdout,
        status: child
            .wait()
            .await
            .map_err(|e| E::Executing(command.to_string(), e.to_string()))?,
    })
}
