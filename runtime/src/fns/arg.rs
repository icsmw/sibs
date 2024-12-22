use crate::*;

#[derive(Debug)]
pub struct FnArgValue {
    pub value: RtValue,
    pub link: SrcLink,
}

#[derive(Debug)]
pub struct FnArgDeclaration {
    pub ty: DataType,
    pub link: SrcLink,
}
