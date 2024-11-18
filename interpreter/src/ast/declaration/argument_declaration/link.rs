use lexer::SrcLink;

use crate::*;

impl From<&ArgumentDeclaration> for SrcLink {
    fn from(node: &ArgumentDeclaration) -> Self {
        let variable: SrcLink = node.variable.as_ref().into();
        let ty: SrcLink = node.r#type.as_ref().into();
        SrcLink {
            from: variable.from,
            to: ty.to,
            src: ty.src,
        }
    }
}
