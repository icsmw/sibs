mod types;

use crate::*;
pub(crate) use types::*;

#[derive(Debug, Default)]
pub struct SemanticCx {
    pub tys: Types,
    pub fns: Fns,
}
