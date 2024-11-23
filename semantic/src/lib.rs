mod ast;
mod data_type;
mod error;
mod type_context;

pub use data_type::*;
use diagnostics::*;
use error::*;
pub use type_context::*;

pub trait InferType {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>>;
}
