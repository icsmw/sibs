use crate::{
    cli,
    error::E,
    functions::Implementation,
    inf::{context::Context, operator::Operator},
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
pub struct Os {
    pub probe: String,
}

impl Implementation<Os, bool> for Os {
    fn from(function: Function, cx: &mut Context) -> Result<Option<Os>, E> {
        if function.name.trim() != NAME {
            return Ok(None);
        }
        let arg = function
            .args
            .as_ref()
            .ok_or(Error::NoArguments)?
            .get(0)
            .ok_or(Error::InvalidNumberOfArguments)?;
        let probe = match &arg {
            Argument::String(value) => value.to_owned(),
            Argument::ValueString(value_string) => String::new(),
            Argument::VariableName(variable) => variable
                .val(cx)?
                .get_as::<String>()
                .ok_or(Error::NoArguments)?
                .to_owned(),
            _ => Err(Error::InvalidArgumentType)?,
        };
        Ok(Some(Os { probe }))
    }

    fn run(&mut self, _context: &mut Context) -> Result<bool, E> {
        Ok(self.probe.to_lowercase() == std::env::consts::OS)
    }
}
