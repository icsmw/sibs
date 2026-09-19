use crate::*;

#[derive(Debug, Clone)]
pub struct InterpreterEnvironment {
    pub rt: Runtime,
    pub cx: ExecutionContext,
    pub job: Job,
}

impl InterpreterEnvironment {
    pub fn new(rt: Runtime, cx: ExecutionContext, job: Job) -> Self {
        Self { rt, cx, job }
    }
    pub fn from_job(&self, job: Job) -> Self {
        Self {
            rt: self.rt.clone(),
            cx: self.cx.clone(),
            job,
        }
    }
}
