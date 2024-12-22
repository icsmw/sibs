use asttree::LinkedNode;

use crate::*;

#[derive(Debug)]
pub enum FnBody {
    Node(LinkedNode),
    Executor(FnExecutor),
}

pub type FnExecutor = fn(RtFnContext) -> RtResult<E>;

#[derive(Debug)]
pub struct FnEntity {
    pub name: String,
    pub args: Vec<FnArgDeclaration>,
    pub result: DataType,
    pub body: FnBody,
}
