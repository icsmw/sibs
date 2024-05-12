mod error;

use crate::inf::{tracker::Job, Context};
use std::{
    path::PathBuf,
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
        .kill_on_drop(true)
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
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| E::Setup(e.to_string()))
}

fn parse_command(command: &str) -> (&str, Vec<&str>) {
    let mut parts = command.split_ascii_whitespace().collect::<Vec<&str>>();
    (parts.remove(0), parts)
}

pub async fn run(
    token: CancellationToken,
    command: &str,
    cwd: &PathBuf,
    cx: Context,
) -> Result<Option<ExitStatus>, E> {
    fn post_logs(line: Result<String, LinesCodecError>, job: &Job) -> String {
        match line {
            Ok(line) => {
                job.output(line.trim_end());
                job.progress(None);
                line
            }
            Err(err) => {
                job.err(format!("Error during decoding command output: {err}",));
                String::new()
            }
        }
    }
    fn chk_status(exit_status: ExitStatus, job: &Job) -> Option<ExitStatus> {
        if exit_status.success() {
            job.success();
        } else {
            job.fail();
        }
        Some(exit_status)
    }
    let job = cx
        .tracker
        .create_job(
            &format!("{}: {}", cx.scenario.to_relative_path(cwd), command),
            None,
        )
        .await?;
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
    select! {
        res = async {
            join!(
                async {
                    while let Some(line) = stdout.next().await {
                        post_logs(line, &job);
                    }
                },
                async {
                    while let Some(line) = stderr.next().await {
                        post_logs(line, &job);
                    }
                }
            );
            child.wait().await
            .map(|status| chk_status(status, &job))
            .map_err(|e| {
                job.fail();
                E::Executing(command.to_string(), e.to_string())
            })
        } => {
            res
        }
        _ = async {
            token.cancelled().await;
        } => {
            match child.try_wait() {
                Ok(Some(status)) => {
                    Ok(chk_status(status, &job))
                }
                Ok(None) => {
                    if let Err(err) = child.kill().await {
                        job.err(format!("fail to kill process: {err}"));
                    } else {
                        job.output("process has been killed");
                    }
                    job.cancelled();
                    Ok(None)
                }
                Err(err) => {
                    job.fail();
                    Err(E::Executing(command.to_string(), err.to_string()))
                }
            }
        }
    }
}
