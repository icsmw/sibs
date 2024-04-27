use crate::{
    executors::{Executor, ExecutorPinnedResult, E},
    inf::any::AnyValue,
    inf::context::Context,
};
use std::path::PathBuf;
use thiserror::Error;

const NAME: &str = "import";

#[derive(Error, Debug)]
pub enum Error {
    #[error("File {0} doesn't exist")]
    NoFile(String),
    #[error("Import function is used only during reading of file")]
    IsNotUsedInRuntime,
}

impl From<Error> for E {
    fn from(e: Error) -> Self {
        E::FunctionExecuting(format!("@{NAME}"), e.to_string())
    }
}

#[derive(Debug)]
pub struct Import {}

impl Import {
    pub fn get(mut path: PathBuf, cwd: PathBuf) -> Result<PathBuf, E> {
        if path.is_relative() {
            path = cwd.join(path);
        }
        if !path.exists() {
            Err(Error::NoFile(path.to_string_lossy().to_string()))?;
        }
        Ok(path)
    }
}
impl Executor for Import {
    fn execute(_: Vec<AnyValue>, _cx: Context) -> ExecutorPinnedResult {
        Box::pin(async { Err(Error::IsNotUsedInRuntime.into()) })
    }

    fn get_name() -> String {
        NAME.to_owned()
    }
}
