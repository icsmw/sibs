use crate::*;

#[derive(Debug)]
pub enum FnArgDeclaration {
    EFn(Ty),
    UFn(UserFnArgDeclaration),
}

#[derive(Debug)]
pub struct FnArgValue {
    pub value: RtValue,
    pub link: SrcLink,
}

impl FnArgValue {
    pub fn new(value: RtValue, link: SrcLink) -> Self {
        Self { value, link }
    }
    pub fn by_node(value: RtValue, node: &LinkedNode) -> Self {
        Self {
            value,
            link: node.md.link.clone(),
        }
    }
}
