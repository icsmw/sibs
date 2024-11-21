use crate::*;
use asttree::*;

impl From<&VariableTypeDeclaration> for SrcLink {
    fn from(node: &VariableTypeDeclaration) -> Self {
        if let Some(l) = node.types.last() {
            let l: SrcLink = l.into();
            SrcLink {
                from: node.token.pos.from,
                to: l.to,
                src: l.src,
            }
        } else {
            SrcLink::default()
        }
    }
}
