mod status;

#[cfg(test)]
mod tests;

use std::{
    path::Path,
    process::{ExitStatus, Stdio},
};
use tokio::{
    join,
    process::{Child, Command},
    select,
};
use tokio_stream::StreamExt;
use tokio_util::codec::{self, LinesCodec, LinesCodecError};

use crate::*;
pub use status::*;

#[cfg(windows)]
fn setup<S: AsRef<str>, P: AsRef<Path>>(cmd: S, cwd: P) -> Result<Child, E> {
    let (cmd, args) = parse_command(cmd.as_ref());
    let cwd_str = cwd.as_ref().to_string_lossy().to_string();
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    Command::new(cmd)
        .args(args)
        .current_dir(cwd)
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| E::SpawnSetup(e.to_string(), cwd_str))
}

#[cfg(not(windows))]
fn setup<S: AsRef<str>, P: AsRef<Path>>(cmd: S, cwd: P) -> Result<Child, E> {
    let (cmd, args) = parse_command(cmd.as_ref());
    let cwd_str = cwd.as_ref().to_string_lossy().to_string();
    Command::new(cmd)
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| E::SpawnSetup(e.to_string(), cwd_str))
}

fn parse_command(cmd: &str) -> (&str, Vec<&str>) {
    let mut parts = cmd.split_ascii_whitespace().collect::<Vec<&str>>();
    (parts.remove(0), parts)
}

pub async fn spawn<S: AsRef<str>, P: AsRef<Path>>(
    cmd: S,
    cwd: P,
    job: Job,
) -> Result<SpawnStatus, E> {
    fn post_logs(
        line: Result<String, LinesCodecError>,
        output: &mut Vec<String>,
        stdout: bool,
        journal: &JobJournal,
        progress: &JobProgress,
    ) {
        match line {
            Ok(line) => {
                let trimmed = line.trim_end();
                progress.msg(trimmed);
                if stdout {
                    journal.stdout(trimmed);
                } else {
                    journal.stderr(trimmed);
                }
                output.push(trimmed.to_owned());
            }
            Err(err) => {
                journal.err(format!("Error during decoding cmd output: {err}",));
            }
        }
    }
    fn get_status(status: ExitStatus, output: Vec<String>) -> SpawnStatus {
        if status.success() {
            SpawnStatus::Success(output)
        } else {
            SpawnStatus::Failed(status.code(), output)
        }
    }
    let cwd_str = cwd.as_ref().to_string_lossy().to_string();
    let mut cstdout = Vec::new();
    let mut cstderr = Vec::new();
    let job = job.child(cmd.as_ref()).await?;
    job.start().started(Some(cmd.as_ref())).await?;
    let journal = job.journal();
    let mut progress = None;
    let result = async {
        progress = Some(job.progress().await?);
        let progress = progress.as_ref().expect("progress registered");
        let mut child = match setup(cmd.as_ref(), cwd) {
            Ok(child) => child,
            Err(err) => {
                return Ok(SpawnStatus::RunError(err.to_string()));
            }
        };
        let mut stdout = codec::FramedRead::new(
            child.stdout.take().ok_or_else(|| {
                E::SpawnSetup(String::from("Fail to get stdout handle"), cwd_str.clone())
            })?,
            LinesCodec::default(),
        );
        let mut stderr = codec::FramedRead::new(
            child.stderr.take().ok_or_else(|| {
                E::SpawnSetup(String::from("Fail to get stderr handle"), cwd_str.clone())
            })?,
            LinesCodec::default(),
        );
        let cancel = job.cancel();
        let status = select! {
            res = async {
                join!(
                    async {
                        while let Some(line) = stdout.next().await {
                            post_logs(line, &mut cstdout, true, &journal, progress)
                        }
                    },
                    async {
                        while let Some(line) = stderr.next().await {
                            post_logs(line, &mut cstderr, false, &journal, progress)
                        }
                    }
                );
                child.wait().await
            } => {
                get_status(
                    res.map_err(|err| E::SpawnError(err.to_string(), cwd_str))?,
                    [cstdout, cstderr].concat(),
                )
            }
            _ = cancel.cancellation() => {
                journal.debug("Cancel signal has been gotten");
                match child.try_wait() {
                    Ok(Some(status)) => {
                        get_status(status, [cstdout, cstderr].concat())
                    }
                    Ok(None) => {
                        child.kill().await.map_err(|err| E::SpawnError(err.to_string(), cwd_str.clone()))?;
                        SpawnStatus::Cancelled
                    }
                    Err(err) => {
                        return Err(E::SpawnError(err.to_string(), cwd_str.clone()));
                    }
                }
            }
        };
        Ok(status)
    }.await;
    match &result {
        Ok(SpawnStatus::Success(_)) => {
            job.done().success::<String>(None).await?;
            if let Some(progress) = &progress {
                progress.success::<String>(None);
            }
        }
        Ok(SpawnStatus::Cancelled) => {
            // An inherited token does not change this job's own state.
            job.cancel().cancel().await?;
            job.cancel().cancelled::<String>(None).await?;
            if let Some(progress) = &progress {
                progress.cancelled::<String>(None);
            }
        }
        other => {
            let message = match other {
                Ok(SpawnStatus::Failed(code, _)) => format!("Finished with error; code: {code:?}"),
                Ok(SpawnStatus::RunError(err)) => err.clone(),
                Err(err) => err.to_string(),
                _ => unreachable!(),
            };
            job.done().failed(Some(&message)).await?;
            if let Some(progress) = &progress {
                progress.failed(Some(&message));
            }
        }
    }
    result
}
