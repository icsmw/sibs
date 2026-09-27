use runtime::{RtParameters, RtValue};

use crate::{runtime, ExecutionOptions, ExecutorError, InterContext, InterpretOwned};

#[derive(Debug)]
pub struct Executor {
    options: ExecutionOptions,
}

impl Executor {
    pub fn new(options: ExecutionOptions) -> Self {
        Self { options }
    }

    pub async fn run(self, ctx: &mut InterContext) -> Result<RtValue, ExecutorError> {
        let (Some(anchor), Some(scx)) = ctx.get_executor_ctx() else {
            return Err(ExecutorError::InvalidContext);
        };
        let params = RtParameters::new(
            self.options.component,
            self.options.task,
            self.options.args,
            self.options.cwd,
        );
        let rt = runtime(params.clone(), scx).map_err(ExecutorError::RuntimeSetup)?;
        let execution = async {
            let env = rt
                .create_interpreter_env(format!("{}:{}", params.component, params.task), None)
                .await
                .map_err(ExecutorError::RuntimeSetup)?;
            anchor
                .interpret_owned(env)
                .await
                .map_err(ExecutorError::Execution)
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
