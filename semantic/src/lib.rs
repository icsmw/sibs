mod ast;
mod error;
mod type_context;

pub(crate) use asttree::*;
pub use common::*;
pub(crate) use diagnostics::*;
pub(crate) use error::*;
pub(crate) use type_context::*;

pub trait InferType {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>>;
}
