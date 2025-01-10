use std::fmt::Debug;

use crate::*;

pub type TaskExecutor = Box<dyn Fn(Runtime) -> RtPinnedResult<'static, LinkedErr<E>> + Send + Sync>;

pub enum TaskBody {
    Node(LinkedNode),
    Executor(Metadata, TaskExecutor),
}

impl Debug for TaskBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Node(n) => format!("TaskBody::Node({n:?})"),
                Self::Executor(..) => "TaskBody::Executor(..)".to_owned(),
            }
        )
    }
}

#[derive(Debug)]
pub struct TaskEntity {
    pub uuid: Uuid,
    pub name: String,
    pub master: MasterComponent,
    pub args: Vec<TaskArgDeclaration>,
    pub result: Ty,
    pub body: TaskBody,
}

impl TaskEntity {
    pub fn args_tys(&self) -> Vec<&Ty> {
        self.args.iter().map(|arg| &arg.ty).collect::<Vec<&Ty>>()
    }
}
