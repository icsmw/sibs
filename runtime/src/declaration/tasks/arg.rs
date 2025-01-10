use crate::*;

#[derive(Debug)]
pub struct TaskArgDeclaration {
    pub ty: Ty,
    pub ident: String,
    pub link: SrcLink,
}
