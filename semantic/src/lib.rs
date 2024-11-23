mod ast;
mod error;
mod type_context;

use diagnostics::*;
use error::*;
pub use type_context::*;

use common::*;

pub trait InferType {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>>;
}
