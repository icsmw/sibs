use crate::*;
use asttree::*;

impl From<&FunctionDeclaration> for SrcLink {
    fn from(node: &FunctionDeclaration) -> Self {
        let block: SrcLink = node.block.as_ref().into();
        SrcLink {
            from: node.sig.pos.from,
            to: block.to,
            src: block.src,
        }
    }
}
