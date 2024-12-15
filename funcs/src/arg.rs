use crate::*;
use lexer::SrcLink;

pub struct FnArg {
    pub value: RtValue,
    pub link: SrcLink,
}
