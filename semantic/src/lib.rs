#[cfg(test)]
mod tests;

mod ast;
mod context;
mod error;

pub(crate) use asttree::*;
pub(crate) use context::*;
pub use context::*;
pub(crate) use diagnostics::*;
pub(crate) use error::*;
pub(crate) use lexer::*;

pub(crate) use std::collections::HashMap;
pub(crate) use uuid::Uuid;

pub trait InferType {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>>;
}

pub trait Initialize {
    fn initialize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>>;
}

pub trait Finalization {
    fn finalize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
