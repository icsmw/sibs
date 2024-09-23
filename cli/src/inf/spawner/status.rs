use std::process::ExitStatus;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SpawnStatus {
    pub code: Option<i32>,
    pub success: bool,
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
    pub error: Option<String>,
    pub cancelled: bool,
}

impl SpawnStatus {
    pub fn from_res(res: Result<ExitStatus, String>) -> Self {
        match res {
            Ok(status) => Self::from_status(status),
            Err(err) => Self {
                code: None,
                error: Some(err),
                ..Default::default()
            },
        }
    }
    pub fn from_status(status: ExitStatus) -> Self {
        Self {
            code: status.code(),
            success: status.success(),
            ..Default::default()
        }
    }
    pub fn stdout(mut self, stdout: Vec<String>) -> Self {
        self.stdout = stdout;
        self
    }
    pub fn stderr(mut self, stderr: Vec<String>) -> Self {
        self.stderr = stderr;
        self
    }
}
