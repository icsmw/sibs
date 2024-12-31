use std::fmt::Debug;

use asttree::LinkedNode;

use crate::*;

pub type ExecutorFn = Box<dyn Fn(Runtime) -> RtPinnedResult<'static, LinkedErr<E>> + Send + Sync>;
pub enum FnBody {
    Node(LinkedNode),
    Executor(Metadata, ExecutorFn),
}

impl Debug for FnBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Node(n) => format!("FnBody::Node({n:?})"),
                Self::Executor(..) => "FnBody::Executor(..)".to_owned(),
            }
        )
    }
}

#[derive(Debug)]
pub struct FnEntity {
    pub uuid: Uuid,
    pub name: String,
    pub args: Vec<FnArgDeclaration>,
    pub result: DataType,
    pub body: FnBody,
}
