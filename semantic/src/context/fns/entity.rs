use asttree::LinkedNode;

use crate::*;

#[derive(Debug)]
pub struct FnEntity {
    pub uuid: Uuid,
    pub name: String,
    pub args: Vec<FnArgDeclaration>,
    pub result: DataType,
    pub node: LinkedNode,
}
