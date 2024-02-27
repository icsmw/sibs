use crate::{
    executors::{Executor, ExecutorPinnedResult, E},
    inf::{any::AnyValue, context::Context, tracker::Logs},
};
use thiserror::Error;

const NAME: &str = "repeat";

#[derive(Error, Debug)]
pub enum Error {
    #[error("No arguments; path required")]
    NoArguments,
    #[error("Expecting 2 arguments: @repeat {{string}} {{count}};")]
    InvalidNumberOfArguments,
    #[error("Invalid argument type; expected string.")]
    InvalidTargetType,
    #[error("Fail to extract string value from argument")]
    InvalidCountType(String),
    #[error("Fail to extract count (usize) from argument")]
    NoStringValue,
    #[error("Fail to extract variable name from VariableName entity")]
    NoVariableName,
    #[error("Fail to get number of repeating")]
    FailToGetRepeatingNumber,
}

impl From<Error> for E {
    fn from(e: Error) -> Self {
        E::FunctionExecuting(format!("@{NAME}"), e.to_string())
    }
}

#[derive(Debug)]
pub struct Repeat {}

impl Executor for Repeat {
    fn execute(args: Vec<AnyValue>, cx: &mut Context) -> ExecutorPinnedResult {
        Box::pin(async move {
            if args.is_empty() {
                Err(Error::NoArguments)?;
            }
            if args.len() != 2 {
                Err(Error::InvalidNumberOfArguments)?;
            }
            let logger = cx.tracker.create_logger(format!("@{NAME}"));
            let target = args[0].get_as_string().ok_or(Error::InvalidTargetType)?;
            let count = args[1]
                .get_as_string()
                .ok_or(Error::InvalidCountType(format!("{:?}", args[1])))?
                .parse::<usize>()
                .map_err(|_| Error::InvalidCountType(format!("{:?}", args[1])))?;
            logger.log(format!("repeating \"{target}\" {count} times;",));
            Ok(AnyValue::new(target.repeat(count)))
        })
    }

    fn get_name() -> String {
        NAME.to_owned()
    }
}
