use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ExecutionOptions {
    pub component: String,
    pub task: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
}

impl ExecutionOptions {
    pub fn new<C: ToString, T: ToString, P: AsRef<Path>>(component: C, task: T, cwd: P) -> Self {
        Self {
            component: component.to_string(),
            task: task.to_string(),
            args: Vec::new(),
            cwd: cwd.as_ref().to_path_buf(),
        }
    }

    pub fn with_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.args = args.into_iter().map(|arg| arg.to_string()).collect();
        self
    }
}
