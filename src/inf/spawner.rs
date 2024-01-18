use crate::inf::{scenario::Scenario, tracker::Tracker};

use async_process::{Command, ExitStatus, Stdio};
use async_std::{io::BufReader, prelude::*};
use futures_lite::{future, FutureExt};
use std::{env::vars, io, path::PathBuf};

#[derive(Clone, Debug)]
pub struct SpawnResult {
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
    pub status: ExitStatus,
    pub job: String,
}

impl SpawnResult {
    pub fn empty() -> Self {
        SpawnResult {
            stdout: vec![],
            stderr: vec![],
            status: ExitStatus::default(),
            job: String::new(),
        }
    }
}

pub async fn spawn(
    command: &str,
    cwd: &PathBuf,
    tracker: &Tracker,
    location: &Scenario,
) -> Result<SpawnResult, io::Error> {
    let mut parts = command.split(' ').collect::<Vec<&str>>();
    let cmd = parts.remove(0);
    #[allow(clippy::useless_vec)]
    let mut child = Command::new(cmd)
        .current_dir(cwd)
        .args(parts)
        .envs(
            vec![
                vars().collect::<Vec<(String, String)>>(),
                vec![(String::from("TERM"), String::from("xterm-256color"))],
            ]
            .concat(),
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let sequence = tracker
        .start(&format!("{}: {cmd}", location.to_relative_path(cwd)), None)
        .await?;
    let mut stdout_lines: Vec<String> = vec![];
    let drain_stdout = {
        let storage = &mut stdout_lines;
        let stdout = child.stdout.take().unwrap();
        async move {
            let mut buf = BufReader::new(stdout);
            loop {
                let mut line = String::new();
                let read_lines = buf.read_line(&mut line).await?;
                if read_lines == 0 {
                    break;
                } else {
                    tracker.msg(sequence, &line).await;
                    tracker.progress(sequence, None).await;
                    storage.push(line);
                }
            }
            future::pending::<()>().await;
            Ok::<Option<ExitStatus>, io::Error>(None)
        }
    };

    let mut stderr_lines: Vec<String> = vec![];
    let drain_stderr = {
        let storage = &mut stderr_lines;
        let stderr = child.stderr.take().unwrap();
        async move {
            let mut buf = BufReader::new(stderr);
            loop {
                let mut line = String::new();
                let read_lines = buf.read_line(&mut line).await?;
                if read_lines == 0 {
                    break;
                } else {
                    tracker.progress(sequence, None).await;
                    if !line.trim().is_empty() {
                        storage.push(line);
                    }
                }
            }
            future::pending::<()>().await;
            Ok::<Option<ExitStatus>, io::Error>(None)
        }
    };
    let status = match drain_stdout
        .or(drain_stderr)
        .or(async move { Ok(Some(child.status().await?)) })
        .await
    {
        Ok(status) => status,
        Err(err) => {
            tracker.fail(sequence, &err.to_string()).await;
            return Err(err);
        }
    };
    if let Some(status) = status {
        if status.success() {
            tracker.success(sequence, "").await;
        } else {
            tracker.fail(sequence, "finished with errors").await;
        }
        Ok(SpawnResult {
            stdout: stdout_lines,
            stderr: stderr_lines,
            status,
            job: cmd.to_owned(),
        })
    } else {
        tracker
            .fail(sequence, "Fail to get exist status of spawned command")
            .await;
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Fail to get exist status of spawned command",
        ))
    }
}
