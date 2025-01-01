use crate::*;

#[derive(Debug)]
pub struct UserFnArgDeclaration {
    pub ty: DataType,
    pub ident: String,
    pub link: SrcLink,
}
