use crate::*;

pub struct FnEnv {
    pub args: Vec<FnArgValue>,
    pub rt: Runtime,
    pub cx: ExecutionContext,
    pub job: Job,
    pub caller: SrcLink,
}

impl FnEnv {
    pub fn from_interpreter_env(
        env: &InterpreterEnvironment,
        args: Vec<FnArgValue>,
        caller: SrcLink,
    ) -> Self {
        Self {
            args,
            rt: env.rt.clone(),
            cx: env.cx.clone(),
            job: env.job.clone(),
            caller,
        }
    }
    pub fn to_interpreter_env(&self, job: Job) -> InterpreterEnvironment {
        InterpreterEnvironment {
            rt: self.rt.clone(),
            cx: self.cx.clone(),
            job,
        }
    }
}
