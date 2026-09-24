use runtime::{RtParameters, RtValue};
use semantic::Script;

use crate::{runtime, ExecutionFailure, ExecutionOptions, ExecutorError, InterpretOwned};

#[derive(Debug)]
pub struct Executor {
    script: Script,
    options: ExecutionOptions,
}

impl Executor {
    pub fn new(script: Script, options: ExecutionOptions) -> Self {
        Self { script, options }
    }

    pub async fn run(self) -> Result<RtValue, ExecutorError> {
        let script = self.script.into_inner();
        let params = RtParameters::new(
            self.options.component,
            self.options.task,
            self.options.args,
            self.options.cwd,
        );
        let rt = runtime(params.clone(), script.scx).map_err(ExecutorError::RuntimeSetup)?;
        let execution = async {
            let env = rt
                .create_interpreter_env(format!("{}:{}", params.component, params.task), None)
                .await
                .map_err(ExecutorError::RuntimeSetup)?;
            script.anchor.interpret_owned(env).await.map_err(|err| {
                ExecutorError::Execution(Box::new(ExecutionFailure::new(script.parser, err)))
            })
        };
        match (execution.await, rt.destroy().await) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(err), Ok(())) => Err(err),
            (Ok(value), Err(err)) => Err(ExecutorError::ValueAndShutdown { value, err }),
            (Err(err), Err(shutdown_err)) => Err(ExecutorError::ErrorAndShutdown {
                err: Box::new(err),
                shutdown_err,
            }),
        }
    }
}
