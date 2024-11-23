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
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            Declaration::ArgumentDeclaration(n) => n.infer_type(tcx),
            Declaration::Closure(n) => n.infer_type(tcx),
            Declaration::FunctionDeclaration(n) => n.infer_type(tcx),
            Declaration::VariableDeclaration(n) => n.infer_type(tcx),
            Declaration::VariableType(n) => n.infer_type(tcx),
            Declaration::VariableTypeDeclaration(n) => n.infer_type(tcx),
            Declaration::VariableVariants(n) => n.infer_type(tcx),
        }
    }
}
