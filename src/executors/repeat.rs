use crate::{
    entry::{Argument, Function},
    executors::{Executor, ExecutorPinnedResult, E},
    inf::{any::AnyValue, context::Context, operator::Operator},
};
use thiserror::Error;

const NAME: &str = "repeat";

#[derive(Error, Debug)]
pub enum Error {
    #[error("No arguments; path required")]
    NoArguments,
    #[error("Only one argument is required: path")]
    InvalidNumberOfArguments,
    #[error("Invalid argument type; expected string.")]
    InvalidArgumentType,
    #[error("Fail to extract string value from PatternString entity")]
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
    fn execute<'a>(function: &'a Function, cx: &'a mut Context) -> ExecutorPinnedResult<'a> {
        Box::pin(async {
            if function.name.trim() != NAME {
                return Ok(None);
            }
            let logger = cx.tracker.get_logger(format!("@{NAME}"));
            let first = function
                .args
                .as_ref()
                .ok_or(Error::NoArguments)?
                .get(0)
                .ok_or(Error::InvalidNumberOfArguments)?;
            let second = function
                .args
                .as_ref()
                .ok_or(Error::NoArguments)?
                .get(1)
                .ok_or(Error::InvalidNumberOfArguments)?;
            let target = match first {
                Argument::SimpleString(s) => s.to_string(),
                Argument::PatternString(value_string) => value_string
                    .process(None, &[], &[], cx)
                    .await?
                    .ok_or(Error::NoStringValue)?
                    .get_as::<String>()
                    .ok_or(Error::InvalidArgumentType)?
                    .to_owned(),
                Argument::VariableName(variable) => variable
                    .process(None, &[], &[], cx)
                    .await?
                    .ok_or(Error::NoVariableName)?
                    .get_as::<String>()
                    .ok_or(Error::InvalidArgumentType)?
                    .to_owned(),
                _ => Err(Error::InvalidArgumentType)?,
            };
            let count = match second {
                Argument::SimpleString(s) => s.to_string(),
                Argument::PatternString(value_string) => value_string
                    .process(None, &[], &[], cx)
                    .await?
                    .ok_or(Error::NoStringValue)?
                    .get_as::<String>()
                    .ok_or(Error::InvalidArgumentType)?
                    .to_owned(),
                Argument::VariableName(variable) => variable
                    .process(None, &[], &[], cx)
                    .await?
                    .ok_or(Error::NoVariableName)?
                    .get_as::<String>()
                    .ok_or(Error::InvalidArgumentType)?
                    .to_owned(),
                _ => Err(Error::InvalidArgumentType)?,
            }
            .parse::<usize>()
            .map_err(|_| Error::FailToGetRepeatingNumber)?;
            logger
                .log(format!("repeating \"{target}\" {count} times;",))
                .await;
            Ok(Some(AnyValue::new(target.repeat(count))))
        })
    }

    fn get_name() -> String {
        NAME.to_owned()
    }
}
