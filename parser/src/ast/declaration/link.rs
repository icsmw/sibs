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
