mod error;

use crate::inf::{tracker::Job, Context};
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

pub async fn run(command: &str, cwd: &PathBuf, cx: Context) -> Result<ExitStatus, E> {
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
    child
        .wait()
        .await
        .map(|res| {
            if res.success() {
                job.success();
            } else {
                job.fail();
            }
            res
        })
        .map_err(|e| {
            job.fail();
            E::Executing(command.to_string(), e.to_string())
        })
}
