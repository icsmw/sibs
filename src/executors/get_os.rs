use crate::{
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
    fn execute<'a>(args: Vec<AnyValue>, cx: &'a mut Context) -> ExecutorPinnedResult<'a> {
        Box::pin(async move {
            let logger = cx.tracker.create_logger(format!("@{NAME}"));
            if !args.is_empty() {
                Err(Error::InvalidNumberOfArguments)?;
            }
            logger
                .log(format!("result \"{}\"", std::env::consts::OS))
                .await;
            Ok(AnyValue::new(std::env::consts::OS.to_string()))
        })
    }

    fn get_name() -> String {
        NAME.to_owned()
    }
}
