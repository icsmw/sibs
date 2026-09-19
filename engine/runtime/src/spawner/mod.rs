mod status;

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
    async fn get_status(
        status: ExitStatus,
        output: Vec<String>,
        job: &Job,
    ) -> Result<SpawnStatus, E> {
        if status.success() {
            job.done().success::<&str>(None).await?;
            Ok(SpawnStatus::Success(output))
        } else {
            job.done()
                .failed(Some(format!(
                    "Finished with error; code: {}",
                    status
                        .code()
                        .map(|c| c.to_string())
                        .unwrap_or("unknown code".to_owned())
                )))
                .await?;
            Ok(SpawnStatus::Failed(status.code(), output))
        }
    }
    let cwd_str = cwd.as_ref().to_string_lossy().to_string();
    let mut cstdout = Vec::new();
    let mut cstderr = Vec::new();
    let job = job.child(cmd.as_ref()).await?;
    let mut child = match setup(cmd, cwd) {
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
    let journal = job.journal();
    let progress = job.progress().await?;
    let cancel = job.cancel();
    let status = select! {
        res = async {
            join!(
                async {
                    while let Some(line) = stdout.next().await {
                        post_logs(line, &mut cstdout, true, &journal, &progress)
                    }
                },
                async {
                    while let Some(line) = stderr.next().await {
                        post_logs(line, &mut cstderr, false, &journal, &progress)
                    }
                }
            );
            child.wait().await
        } => {
            get_status(
                res.map_err(|err| E::SpawnError(err.to_string(), cwd_str))?,
                [cstdout, cstderr].concat(),
                &job
            ).await?
        }
        _ = cancel.cancellation() => {
            journal.debug("Cancel signal has been gotten");
            match child.try_wait() {
                Ok(Some(status)) => {
                    let status = get_status(status, [cstdout, cstderr].concat(), &job).await?;
                    status
                }
                Ok(None) => {
                    if let Err(err) = child.kill().await {
                        job.cancel().cancelled(Some(err.to_string())).await?;
                    } else {
                        job.cancel().cancelled::<String>(None).await?;
                    }
                    SpawnStatus::Cancelled
                }
                Err(err) => {
                    job.cancel().cancelled(Some(format!("Fail to kill process: {err}"))).await?;
                    SpawnStatus::Cancelled
                }
            }
        }
    };
    Ok(status)
}
