use crate::*;

#[derive(Debug)]
pub struct UserFnArgDeclaration {
    pub ty: Ty,
    pub ident: String,
    pub link: SrcLink,
}
