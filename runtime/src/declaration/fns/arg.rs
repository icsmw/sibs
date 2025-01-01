use crate::*;

#[derive(Debug)]
pub enum FnArgDeclaration {
    EFn(DataType),
    UFn(UserFnArgDeclaration),
}
