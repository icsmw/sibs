use crate::{
    functions::Implementation,
    inf::context::Context,
    reader::{
        entry::{Argument, Function},
        error::E,
    },
};
use std::{fs, path::PathBuf};
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
    fn from(val: Error) -> Self {
        E::FunctionError(NAME.to_string(), val.to_string())
    }
}

#[derive(Debug)]
pub struct Import {
    pub path: PathBuf,
}

impl Implementation<Import, String> for Import {
    fn from(function: Function, cx: &mut Context) -> Result<Option<Import>, E> {
        if function.name.trim() != NAME {
            return Ok(None);
        }
        let cwd = cx.cwd.as_ref().ok_or(Error::NoCurrentWorkingFolder)?;
        let args = function.args.ok_or(Error::NoArguments)?;
        if args.args.len() != 1 {
            Err(Error::InvalidNumberOfArguments)?;
        }
        let mut path = if let (_, Argument::String(value)) = &args.args[0] {
            PathBuf::from(value)
        } else {
            Err(Error::InvalidPathArgument)?
        };
        if path.is_relative() {
            path = cwd.join(path);
        }
        if !path.exists() {
            Err(Error::NoFile(path.to_string_lossy().to_string()))?;
        }
        Ok(Some(Import { path }))
    }

    fn run(&mut self, context: &mut Context) -> Result<String, E> {
        fs::read_to_string(&self.path).map_err(|e| e.into())
    }
}
