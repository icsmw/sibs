mod argument_declaration;
mod closure;
mod function_declaration;
mod include_declaration;
mod module_declaration;
mod variable_declaration;
mod variable_name;
mod variable_type;
mod variable_type_declaration;
mod variable_variants;

use crate::*;

impl InferType for Declaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        match self {
            Declaration::ArgumentDeclaration(n) => n.infer_type(scx),
            Declaration::Closure(n) => n.infer_type(scx),
            Declaration::FunctionDeclaration(n) => n.infer_type(scx),
            Declaration::VariableDeclaration(n) => n.infer_type(scx),
            Declaration::VariableType(n) => n.infer_type(scx),
            Declaration::VariableTypeDeclaration(n) => n.infer_type(scx),
            Declaration::VariableVariants(n) => n.infer_type(scx),
            Declaration::VariableName(n) => n.infer_type(scx),
            Declaration::ModuleDeclaration(n) => n.infer_type(scx),
            Declaration::IncludeDeclaration(n) => n.infer_type(scx),
        }
    }
}

impl Initialize for Declaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Declaration::ArgumentDeclaration(n) => n.initialize(scx),
            Declaration::Closure(n) => n.initialize(scx),
            Declaration::FunctionDeclaration(n) => n.initialize(scx),
            Declaration::VariableDeclaration(n) => n.initialize(scx),
            Declaration::VariableType(n) => n.initialize(scx),
            Declaration::VariableTypeDeclaration(n) => n.initialize(scx),
            Declaration::VariableVariants(n) => n.initialize(scx),
            Declaration::VariableName(n) => n.initialize(scx),
            Declaration::ModuleDeclaration(n) => n.initialize(scx),
            Declaration::IncludeDeclaration(n) => n.initialize(scx),
        }
    }
}

impl Finalization for Declaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            Declaration::ArgumentDeclaration(n) => n.finalize(scx),
            Declaration::Closure(n) => n.finalize(scx),
            Declaration::FunctionDeclaration(n) => n.finalize(scx),
            Declaration::VariableDeclaration(n) => n.finalize(scx),
            Declaration::VariableType(n) => n.finalize(scx),
            Declaration::VariableTypeDeclaration(n) => n.finalize(scx),
            Declaration::VariableVariants(n) => n.finalize(scx),
            Declaration::VariableName(n) => n.finalize(scx),
            Declaration::ModuleDeclaration(n) => n.finalize(scx),
            Declaration::IncludeDeclaration(n) => n.finalize(scx),
        }
    }
}
