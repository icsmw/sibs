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
use tokio_util::{
    codec::{self, LinesCodec, LinesCodecError},
    sync::CancellationToken,
};

use crate::*;
pub use status::*;

#[cfg(windows)]
fn setup<S: AsRef<str>, P: AsRef<Path>>(cmd: S, cwd: P) -> Result<Child, E> {
    let (cmd, args) = parse_command(cmd.as_ref());
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
        .map_err(|e| E::SpawnSetup(e.to_string()))
}

#[cfg(not(windows))]
fn setup<S: AsRef<str>, P: AsRef<Path>>(cmd: S, cwd: P) -> Result<Child, E> {
    let (cmd, args) = parse_command(cmd.as_ref());
    Command::new(cmd)
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| E::SpawnSetup(e.to_string()))
}

fn parse_command(cmd: &str) -> (&str, Vec<&str>) {
    let mut parts = cmd.split_ascii_whitespace().collect::<Vec<&str>>();
    (parts.remove(0), parts)
}

pub async fn spawn<S: AsRef<str>, P: AsRef<Path>>(
    cmd: S,
    cwd: P,
    owner: Uuid,
    parent: Option<Uuid>,
    token: CancellationToken,
    rt: Runtime,
) -> Result<SpawnStatus, E> {
    fn post_logs(
        line: Result<String, LinesCodecError>,
        stdout: bool,
        progress: &Progress,
        journal: &Journal,
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
            }
            Err(err) => {
                journal.err(format!("Error during decoding cmd output: {err}",));
            }
        }
    }
    fn get_status(status: ExitStatus, progress: &Progress, journal: &Journal) -> SpawnStatus {
        if status.success() {
            progress.success::<&str>(None);
            journal.debug("Finished successfully");
            SpawnStatus::Success
        } else {
            progress.failed::<&str>(None);
            journal.debug(format!(
                "Finished with error; code: {}",
                status
                    .code()
                    .map(|c| c.to_string())
                    .unwrap_or("unknown code".to_owned())
            ));
            SpawnStatus::Failed(status.code())
        }
    }
    let journal = rt.journal.owned(owner);
    let progress = rt
        .progress
        .create_job(cmd.as_ref(), parent.as_ref())
        .await?;
    let mut child = setup(cmd, cwd)?;
    let mut stdout = codec::FramedRead::new(
        child
            .stdout
            .take()
            .ok_or_else(|| E::SpawnSetup(String::from("Fail to get stdout handle")))?,
        LinesCodec::default(),
    );
    let mut stderr = codec::FramedRead::new(
        child
            .stderr
            .take()
            .ok_or_else(|| E::SpawnSetup(String::from("Fail to get stderr handle")))?,
        LinesCodec::default(),
    );
    let status = select! {
        res = async {
            join!(
                async {
                    while let Some(line) = stdout.next().await {
                        post_logs(line, true, &progress, &journal)
                    }
                },
                async {
                    while let Some(line) = stderr.next().await {
                         post_logs(line, false, &progress, &journal)
                    }
                }
            );
            child.wait().await
        } => {
            res.map(|status| get_status(status, &progress, &journal))
                .map_err(|err| E::SpawnError(err.to_string()))?
        }
        _ = async {
            token.cancelled().await;
        } => {
            journal.debug("Cancel signal has been gotten");
            match child.try_wait() {
                Ok(Some(status)) => {
                    get_status(status, &progress, &journal)
                }
                Ok(None) => {
                    if let Err(err) = child.kill().await {
                        progress.cancelled(Some(err.to_string()));
                        journal.err(format!("Fail to kill process: {err}"));
                    } else {
                        progress.cancelled(Some(String::from("Process has been killed")));
                        journal.err("Process has been killed");
                    }
                    SpawnStatus::Cancelled
                }
                Err(err) => {
                    progress.cancelled(Some(err.to_string()));
                    journal.err(format!("Fail to kill process: {err}"));
                    SpawnStatus::Cancelled
                }
            }
        }
    };
    Ok(status)
}
