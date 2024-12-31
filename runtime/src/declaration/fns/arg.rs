use crate::*;

#[derive(Debug)]
pub struct FnArgDeclaration {
    pub ty: DataType,
    pub ident: String,
    pub link: SrcLink,
}
