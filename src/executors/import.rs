use crate::{
    executors::{Executor, ExecutorPinnedResult, E},
    inf::any::AnyValue,
    inf::context::Context,
    reader::entry::{Argument, Function},
};
use std::path::PathBuf;
use thiserror::Error;

const NAME: &str = "import";

#[derive(Error, Debug)]
pub enum Error {
    #[error("No arguments; path required")]
    NoArguments,
    #[error("Only one argument is required: path")]
    InvalidNumberOfArguments,
    #[error("As path expected string value")]
    InvalidPathArgument,
    #[error("File {0} doesn't exist")]
    NoFile(String),
    #[error("Import action required CWD")]
    NoCurrentWorkingFolder,
}

impl From<Error> for E {
    fn from(e: Error) -> Self {
        E::FunctionExecuting(format!("@{NAME}"), e.to_string())
    }
}

#[derive(Debug)]
pub struct Import {}

impl Executor for Import {
    fn execute<'a>(function: &'a Function, cx: &'a mut Context) -> ExecutorPinnedResult<'a> {
        Box::pin(async {
            if function.name.trim() != NAME {
                return Ok(None);
            }
            let cwd = cx.cwd.as_ref().ok_or(Error::NoCurrentWorkingFolder)?;
            let args = function.args.as_ref().ok_or(Error::NoArguments)?;
            if args.args.len() != 1 {
                Err(Error::InvalidNumberOfArguments)?;
            }
            let mut path = if let Argument::SimpleString(s) = &args.args[0] {
                PathBuf::from(s.to_string())
            } else {
                Err(Error::InvalidPathArgument)?
            };
            if path.is_relative() {
                path = cwd.join(path);
            }
            if !path.exists() {
                Err(Error::NoFile(path.to_string_lossy().to_string()))?;
            }
            Ok(Some(AnyValue::new(path)))
        })
    }

    fn get_name() -> String {
        NAME.to_owned()
    }
}
