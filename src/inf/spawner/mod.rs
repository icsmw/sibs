mod error;

use crate::inf::tracker::Task;

use futures_lite::{future, FutureExt};
use std::{
    env::vars,
    path::PathBuf,
    process::{ExitStatus, Stdio},
};
use tokio::{
    process::{Child, ChildStderr, ChildStdout, Command},
    select,
};
use tokio_stream::StreamExt;
use tokio_util::codec::{self, FramedRead, LinesCodec};

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
    let mut parts = command.split(' ').collect::<Vec<&str>>();
    (parts.remove(0), parts)
}

pub async fn run(command: &str, cwd: &PathBuf, task: &Task) -> Result<ExitStatus, E> {
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
    while let Some(line) = select! {
        res = stdout.next() => res,
        res = stderr.next() => res
    } {
        let line = line.map_err(|e| E::Output(e.to_string()))?;
        task.msg(line.trim_end());
        task.progress(None);
    }
    Ok(child
        .wait()
        .await
        .map_err(|e| E::Executing(command.to_string(), e.to_string()))?)
}
