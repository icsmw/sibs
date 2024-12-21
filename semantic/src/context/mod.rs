mod fns;
mod types;

pub(crate) use fns::*;
pub(crate) use types::*;

#[derive(Debug, Default)]
pub struct SemanticCx {
    pub tys: Types,
    pub fns: Fns,
}
