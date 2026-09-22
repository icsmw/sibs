use super::*;
use crate::*;
use std::process::{ExitStatus, Stdio};
use tokio::{
    join,
    process::{Child, Command},
    select,
    time::{timeout_at, Duration, Instant},
};
use tokio_stream::StreamExt;
use tokio_util::{
    codec::{self, LinesCodec, LinesCodecError},
    sync::WaitForCancellationFutureOwned,
};

const PROCESS_CLEANUP_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Default)]
pub(super) struct Spawn {
    cmd: String,
    args: Vec<String>,
    cwd: PathBuf,
    process: Option<Child>,
}

impl Spawn {
    pub(super) fn cmd(mut self, cmd: String) -> Self {
        self.cmd = cmd;
        self
    }
    pub(super) fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
    pub(super) fn cwd(mut self, cwd: PathBuf) -> Self {
        self.cwd = cwd;
        self
    }
    #[cfg(windows)]
    fn setup(&self) -> Result<Child, E> {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let mut process = Command::new(&self.cmd)
            .args(&self.args)
            .current_dir(&self.cwd)
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| E::SpawnSetup(e.to_string()))?;
        // No input is supplied by this API. Send EOF before draining output.
        drop(process.stdin.take());
        Ok(process)
    }

    #[cfg(not(windows))]
    fn setup(&self) -> Result<Child, E> {
        let mut process = Command::new(&self.cmd)
            .args(&self.args)
            .current_dir(&self.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| E::SpawnSetup(e.to_string()))?;
        // No input is supplied by this API. Send EOF before draining output.
        drop(process.stdin.take());
        Ok(process)
    }

    pub(super) async fn spawn(
        &mut self,
        cancellation: WaitForCancellationFutureOwned,
        journal: JobJournal,
        progress: JobProgress,
    ) -> Result<SpawnStatus, E> {
        fn get_status(status: ExitStatus, output: Vec<String>) -> SpawnStatus {
            if status.success() {
                SpawnStatus::Success(output)
            } else {
                SpawnStatus::Failed(status.code(), output)
            }
        }
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
        let mut process = match self.setup() {
            Ok(process) => process,
            Err(err) => return Ok(SpawnStatus::RunError(err.to_string())),
        };
        progress.working(Some("starting..."));
        let status = async {
            let mut stdout = codec::FramedRead::new(
                process
                    .stdout
                    .take()
                    .ok_or_else(|| E::SpawnSetup(String::from("Fail to get stdout handle")))?,
                LinesCodec::default(),
            );
            let mut stderr = codec::FramedRead::new(
                process
                    .stderr
                    .take()
                    .ok_or_else(|| E::SpawnSetup(String::from("Fail to get stderr handle")))?,
                LinesCodec::default(),
            );
            let mut cstdout = Vec::new();
            let mut cstderr = Vec::new();
            select! {
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
                    process.wait().await
                } => {
                    Ok(get_status(
                        res.map_err(|err| E::SpawnError(err.to_string()))?,
                        [cstdout, cstderr].concat(),
                    ))
                }
                _ = cancellation => {
                    match process.try_wait() {
                        Ok(Some(status)) => {
                            Ok(get_status(status, [cstdout, cstderr].concat()))
                        }
                        Ok(None) => {
                            Err(E::Cancelled)
                        }
                        Err(err) => {
                            Err(E::SpawnError(err.to_string()))
                        }
                    }
                }
            }
        }
        .await;
        self.process = Some(process);
        status
    }

    pub(super) async fn shutdown(&mut self) -> std::io::Result<()> {
        let Some(mut process) = self.process.take() else {
            return Ok(());
        };
        if matches!(process.try_wait(), Ok(Some(_))) {
            return Ok(());
        }

        let timeout = Instant::now() + PROCESS_CLEANUP_TIMEOUT;

        let result = match process.start_kill() {
            Ok(()) => match timeout_at(timeout, process.wait()).await {
                Ok(result) => result.map(|_| ()),
                Err(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Process cleanup timeout exceeded; termination is unconfirmed",
                )),
            },
            Err(err) => Err(err),
        };
        match result {
            Ok(()) => Ok(()),
            Err(err) => match process.try_wait() {
                Ok(Some(_)) => Ok(()),
                Ok(None) => Err(err),
                Err(wait) => Err(std::io::Error::new(
                    err.kind(),
                    format!("{err}; checking process termination also failed: {wait}"),
                )),
            },
        }
    }
}
