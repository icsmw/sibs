mod error;
mod status;

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
pub use status::*;

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
        .map_err(|e| {
            println!(">>>>>>>>>>>>>>>>> Ooops: {:?}", std::env::current_dir());
            E::Setup(e.to_string())
        })
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
) -> Result<SpawnStatus, E> {
    fn post_logs(line: Result<String, LinesCodecError>, job: &Job) -> Option<String> {
        match line {
            Ok(line) => {
                job.output(line.trim_end());
                job.progress(None);
                Some(line)
            }
            Err(err) => {
                job.err(format!("Error during decoding command output: {err}",));
                None
            }
        }
    }
    fn chk_status(status: ExitStatus, job: &Job) -> ExitStatus {
        if status.success() {
            job.success();
        } else {
            job.fail();
        }
        status
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
    let mut stdout_collected: Vec<String> = Vec::new();
    let mut stderr_collected: Vec<String> = Vec::new();
    let status = select! {
        res = async {
            join!(
                async {
                    while let Some(line) = stdout.next().await {
                        post_logs(line, &job).map(|l| stdout_collected.push(l));
                    }
                },
                async {
                    while let Some(line) = stderr.next().await {
                        post_logs(line, &job).map(|l| stderr_collected.push(l));
                    }
                }
            );
            child.wait().await
            .map(|status| chk_status(status, &job))
            .map_err(|e| {
                job.fail();
                e.to_string()
            })
        } => {
            SpawnStatus::from_res(res)
        }
        _ = async {
            token.cancelled().await;
        } => {
            match child.try_wait() {
                Ok(Some(status)) => {
                    SpawnStatus::from_status(chk_status(status, &job))
                }
                Ok(None) => {
                    if let Err(err) = child.kill().await {
                        job.err(format!("fail to kill process: {err}"));
                    } else {
                        job.output("process has been killed");
                    }
                    job.cancelled();
                    SpawnStatus { cancelled: true, ..Default::default()}
                }
                Err(err) => {
                    job.fail();
                    SpawnStatus { cancelled: true, error: Some(err.to_string()), ..Default::default()}
                }
            }
        }
    };
    Ok(status.stdout(stdout_collected).stderr(stderr_collected))
}
