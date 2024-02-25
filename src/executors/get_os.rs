use crate::{
    entry::Function,
    executors::{Executor, ExecutorPinnedResult, E},
    inf::{any::AnyValue, context::Context, tracker::Logs},
};
use thiserror::Error;

const NAME: &str = "get_os";

#[derive(Error, Debug)]
pub enum Error {
    #[error("GetOs function doesn't expect any arguments")]
    InvalidNumberOfArguments,
}

impl From<Error> for E {
    fn from(e: Error) -> Self {
        E::FunctionExecuting(format!("@{NAME}"), e.to_string())
    }
}

#[derive(Debug)]
pub struct GetOs {}

impl Executor for GetOs {
    fn execute<'a>(function: &'a Function, cx: &'a mut Context) -> ExecutorPinnedResult<'a> {
        Box::pin(async {
            if function.name.trim() != NAME {
                return Ok(None);
            }
            let logger = cx.tracker.create_logger(format!("@{NAME}"));
            if function.args.is_some() {
                Err(Error::InvalidNumberOfArguments)?;
            }
            logger
                .log(format!("result \"{}\"", std::env::consts::OS))
                .await;
            Ok(Some(AnyValue::new(std::env::consts::OS.to_string())))
        })
    }

    fn get_name() -> String {
        NAME.to_owned()
    }
}
