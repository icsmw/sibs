mod spawn;
mod status;

#[cfg(test)]
mod tests;

use std::path::Path;

use crate::*;
use spawn::*;
pub use status::*;

pub struct SpawnerBuilder {
    cmd: String,
    args: Vec<String>,
    cwd: PathBuf,
    job: Job,
}
impl SpawnerBuilder {
    pub async fn new<S: AsRef<str>, P: AsRef<Path>>(
        cmd: S,
        cwd: P,
        parent: Job,
    ) -> Result<Self, E> {
        if cmd.as_ref().trim().is_empty() {
            return Err(E::SpawnEmptyCommand);
        }
        let mut parts = cmd.as_ref().split_ascii_whitespace();
        Ok(Self {
            cmd: parts.next().unwrap_or_default().to_owned(),
            args: parts.map(str::to_owned).collect(),
            cwd: cwd.as_ref().to_path_buf(),
            job: parent.child(cmd.as_ref(), JobVisibility::Visible).await?,
        })
    }

    pub async fn spawn(self) -> Result<SpawnStatus, E> {
        self.job.start().started(Some(&self.cmd)).await?;

        let journal = self.job.journal();
        let progress = match self.job.progress().await {
            Ok(progress) => progress,
            Err(err) => {
                self.job.done().failed::<String>(None).await?;
                return Err(err);
            }
        };

        let mut spawner = Spawn::default()
            .cmd(self.cmd.clone())
            .args(self.args.to_vec())
            .cwd(self.cwd.clone());

        let spawning = spawner
            .spawn(
                self.job.cancel().cancellation_owned(),
                journal.clone(),
                progress.clone(),
            )
            .await;

        let mut state_err = Ok(());
        if matches!(spawning, Err(E::Cancelled)) {
            state_err = self.job.cancel().cancelling().await;
            progress.working(Some("cancelling..."));
        }

        let shutdown = spawner.shutdown().await;

        state_err?;

        match (spawning, shutdown) {
            (Ok(result), shutdown) => {
                if let Err(err) = shutdown {
                    journal.err(format!("Fail to kill process: {err}"));
                }
                match &result {
                    SpawnStatus::Success(_) => {
                        self.job.done().success::<String>(None).await?;
                        progress.success::<String>(None);
                    }
                    SpawnStatus::Failed(..) | SpawnStatus::RunError(..) => {
                        self.job.done().failed::<String>(None).await?;
                        progress.failed::<String>(None);
                    }
                }
                Ok(result)
            }
            (Err(err), shutdown) => {
                match &err {
                    E::Cancelled => {
                        if let Err(err) = shutdown {
                            self.job
                                .done()
                                .failed(Some(format!(
                                    "Fail to kill process on cancellation: {err}"
                                )))
                                .await?;
                            progress.failed::<String>(None);
                        } else {
                            self.job.cancel().cancelled::<String>(None).await?;
                            progress.cancelled::<String>(None);
                        }
                    }
                    _ => {
                        if let Err(err) = shutdown {
                            journal.err(format!("Fail to kill process: {err}"));
                        }
                        self.job.done().failed::<String>(None).await?;
                        progress.failed::<String>(None);
                    }
                }
                Err(err)
            }
        }
    }
}
