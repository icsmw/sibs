use crate::{
    executors::{Executor, ExecutorPinnedResult, E},
    inf::{any::AnyValue, context::Context, operator::Operator},
    reader::entry::{Argument, Function},
};
use thiserror::Error;

const NAME: &str = "os";

#[derive(Error, Debug)]
pub enum Error {
    #[error("No arguments; path required")]
    NoArguments,
    #[error("Only one argument is required: path")]
    InvalidNumberOfArguments,
    #[error("Invalid argument type; expected string.")]
    InvalidArgumentType,
    #[error("Fail to extract string value from ValueString entity")]
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
            let logger = cx.tracker.get_logger(format!("@{NAME}"));
            let arg = function
                .args
                .as_ref()
                .ok_or(Error::NoArguments)?
                .get(0)
                .ok_or(Error::InvalidNumberOfArguments)?;
            let probe = match arg {
                Argument::String(value) => value.to_owned(),
                Argument::ValueString(value_string) => value_string
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
