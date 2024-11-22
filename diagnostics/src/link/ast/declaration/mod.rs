mod argument_declaration;
mod closure;
mod function_declaration;
mod variable_declaration;
mod variable_type;
mod variable_type_declaration;
mod variable_variants;

use crate::*;
use asttree::*;

impl From<&Declaration> for SrcLink {
    fn from(node: &Declaration) -> Self {
        match node {
            Declaration::ArgumentDeclaration(n) => n.into(),
            Declaration::Closure(n) => n.into(),
            Declaration::FunctionDeclaration(n) => n.into(),
            Declaration::VariableDeclaration(n) => n.into(),
            Declaration::VariableType(n) => n.into(),
            Declaration::VariableTypeDeclaration(n) => n.into(),
            Declaration::VariableVariants(n) => n.into(),
        }
    }
}
