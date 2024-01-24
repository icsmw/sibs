use crate::{
    cli,
    error::E,
    functions::{Executor, ExecutorPinnedResult},
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
}

impl From<Error> for E {
    fn from(e: Error) -> Self {
        E {
            sig: format!("@{NAME}"),
            msg: e.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Os {}

impl Executor for Os {
    fn from<'a>(function: &'a mut Function, cx: &'a mut Context) -> ExecutorPinnedResult<'a> {
        Box::pin(async { Ok(None) })

        // if function.name.trim() != NAME {
        //     return Ok(None);
        // }
        // let arg = function
        //     .args
        //     .as_mut()
        //     .ok_or(Error::NoArguments)?
        //     .get_mut(0)
        //     .ok_or(Error::InvalidNumberOfArguments)?;
        // let probe = match arg {
        //     Argument::String(value) => value.to_owned(),
        //     Argument::ValueString(value_string) => value_string
        //         .val(cx)?
        //         .get_as::<String>()
        //         .ok_or(Error::InvalidArgumentType)?
        //         .to_owned(),
        //     Argument::VariableName(variable) => variable
        //         .val(cx)?
        //         .get_as::<String>()
        //         .ok_or(Error::InvalidArgumentType)?
        //         .to_owned(),
        //     _ => Err(Error::InvalidArgumentType)?,
        // };
        // Ok(Some(AnyValue::new(
        //     probe.to_lowercase() == std::env::consts::OS,
        // )))
    }

    fn get_name() -> String {
        NAME.to_owned()
    }
}
