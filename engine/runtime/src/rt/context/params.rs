use std::path::{Path, PathBuf};

use crate::E;

#[derive(Debug, Default, Clone)]
pub struct RtParameters {
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub component: String,
    pub task: String,
}

impl RtParameters {
    pub fn new<S: AsRef<str>, P: AsRef<Path>>(
        component: S,
        task: S,
        args: Vec<String>,
        cwd: P,
    ) -> Self {
        Self {
            args,
            cwd: cwd.as_ref().to_path_buf(),
            component: component.as_ref().to_owned(),
            task: task.as_ref().to_owned(),
        }
    }

    pub fn default_from_cwd() -> Result<Self, E> {
        Ok(Self {
            cwd: std::env::current_dir()?,
            ..Default::default()
        })
    }
}
