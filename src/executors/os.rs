use crate::{
    entry::{Argument, Function},
    executors::{Executor, ExecutorPinnedResult, E},
    inf::{any::AnyValue, context::Context, operator::Operator, tracker::Logs},
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
    #[error("Fail to extract string value from PatternString entity")]
    NoStringValue,
    #[error("Fail to extract variable name from VariableName entity")]
    NoVariableName,
}

impl From<Error> for E {
    fn from(e: Error) -> Self {
        E::FunctionExecuting(format!("@{NAME}"), e.to_string())
    }
}

#[derive(Debug)]
pub struct Os {}

impl Executor for Os {
    fn execute<'a>(function: &'a Function, cx: &'a mut Context) -> ExecutorPinnedResult<'a> {
        Box::pin(async {
            if function.name.trim() != NAME {
                return Ok(None);
            }
            let logger = cx.tracker.create_logger(format!("@{NAME}"));
            let arg = function
                .args
                .as_ref()
                .ok_or(Error::NoArguments)?
                .get(0)
                .ok_or(Error::InvalidNumberOfArguments)?;
            let probe = match arg {
                Argument::SimpleString(s) => s.to_string(),
                Argument::PatternString(value_string) => value_string
                    .execute(None, &[], &[], cx)
                    .await?
                    .ok_or(Error::NoStringValue)?
                    .get_as::<String>()
                    .ok_or(Error::InvalidArgumentType)?
                    .to_owned(),
                Argument::VariableName(variable) => variable
                    .execute(None, &[], &[], cx)
                    .await?
                    .ok_or(Error::NoVariableName)?
                    .get_as::<String>()
                    .ok_or(Error::InvalidArgumentType)?
                    .to_owned(),
                _ => Err(Error::InvalidArgumentType)?,
            };
            logger
                .log(format!(
                    "checking for \"{}\"; result: {}",
                    probe.to_lowercase(),
                    probe.to_lowercase() == std::env::consts::OS
                ))
                .await;
            Ok(Some(AnyValue::new(
                probe.to_lowercase() == std::env::consts::OS,
            )))
        })
    }

    fn get_name() -> String {
        NAME.to_owned()
    }
}
