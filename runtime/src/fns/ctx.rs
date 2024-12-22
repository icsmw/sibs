use crate::*;

pub struct RtFnContext {
    pub args: Vec<FnArgValue>,
    pub ctx: Runtime,
    pub link: SrcLink,
}
