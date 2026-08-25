use runtime::{RtParameters, RtValue};
use semantic::Script;
use uuid::Uuid;

use crate::{runtime, ExecutionFailure, ExecutionOptions, ExecutorError, Interpret};

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
        let env = rt
            .create_interpreter_env(
                Uuid::new_v4(),
                format!("{}:{}", params.component, params.task),
                None,
            )
            .await
            .map_err(ExecutorError::RuntimeSetup)?;
        let result = script.anchor.interpret(env).await;
        let shutdown = rt.destroy().await;

        match (result, shutdown) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(err), Ok(())) => Err(ExecutorError::Execution(Box::new(ExecutionFailure::new(
                script.parser,
                err,
            )))),
            (Ok(value), Err(err)) => Err(ExecutorError::ValueAndShutdown { value, err }),
            (Err(err), Err(shutdown_err)) => {
                tracing::error!("runtime shutdown failed after execution error: {shutdown_err}");
                Err(ExecutorError::Execution(Box::new(ExecutionFailure::new(
                    script.parser,
                    err,
                ))))
            }
        }
    }
}
