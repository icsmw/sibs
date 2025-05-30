#[cfg(test)]
mod tests;

mod ast;
mod context;
mod error;
mod token;

pub(crate) use asttree::*;
pub use context::*;
pub(crate) use diagnostics::*;
pub use error::E as SemanticError;
pub(crate) use error::*;
pub(crate) use runtime::*;
pub use token::*;
pub(crate) use uuid::Uuid;

pub trait InferType {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>>;
}

pub trait Initialize {
    fn initialize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>>;
}

pub trait Finalization {
    fn finalize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}
