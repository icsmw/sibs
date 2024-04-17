use crate::{
    executors::{Executor, ExecutorPinnedResult, E},
    inf::{any::AnyValue, context::Context},
};
use thiserror::Error;

const NAME: &str = "os";

#[derive(Error, Debug)]
pub enum Error {
    #[error("No arguments; path required")]
    NoArguments,
    #[error("Expecting string as a single argument; ex: @os linux;")]
    InvalidNumberOfArguments,
    #[error("Invalid argument type; expected string.")]
    InvalidArgumentType,
}

impl From<Error> for E {
    fn from(e: Error) -> Self {
        E::FunctionExecuting(format!("@{NAME}"), e.to_string())
    }
}

#[derive(Debug)]
pub struct Os {}

impl Executor for Os {
    fn execute(args: Vec<AnyValue>, cx: Context) -> ExecutorPinnedResult {
        Box::pin(async move {
            let logger = cx.journal.owned(format!("@{NAME}"));
            if args.is_empty() {
                Err(Error::NoArguments)?;
            }
            if args.len() != 1 {
                Err(Error::InvalidNumberOfArguments)?;
            }
            let probe = args[0]
                .get_as::<String>()
                .ok_or(Error::InvalidArgumentType)?;
            logger.debug(format!(
                "checking for \"{}\"; result: {}",
                probe.to_lowercase(),
                probe.to_lowercase() == std::env::consts::OS
            ));
            Ok(AnyValue::new(probe.to_lowercase() == std::env::consts::OS))
        })
    }

    fn get_name() -> String {
        NAME.to_owned()
    }
}
