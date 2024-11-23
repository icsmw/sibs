mod argument_declaration;
mod closure;
mod function_declaration;
mod variable_declaration;
mod variable_type;
mod variable_type_declaration;
mod variable_variants;

use crate::*;
use asttree::*;
use diagnostics::*;

impl InferType for Declaration {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        Ok(DataType::Empty)
    }
}
