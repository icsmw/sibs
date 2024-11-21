use crate::*;
use asttree::*;

impl From<&VariableDeclaration> for SrcLink {
    fn from(node: &VariableDeclaration) -> Self {
        if let (Some(ty), None) = (node.r#type.as_ref(), node.assignation.as_ref()) {
            let ty: SrcLink = ty.into();
            SrcLink {
                from: node.token.pos.from,
                to: ty.to,
                src: ty.src,
            }
        } else if let Some(assign) = node.assignation.as_ref() {
            let assign: SrcLink = assign.into();
            SrcLink {
                from: node.token.pos.from,
                to: assign.to,
                src: assign.src,
            }
        } else {
            let variable: SrcLink = node.variable.as_ref().into();
            SrcLink {
                from: node.token.pos.from,
                to: variable.to,
                src: variable.src,
            }
        }
    }
}
