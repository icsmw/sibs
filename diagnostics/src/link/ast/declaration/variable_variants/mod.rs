use crate::*;
use asttree::*;

impl From<&VariableVariants> for SrcLink {
    fn from(node: &VariableVariants) -> Self {
        if let Some(l) = node.variants.last() {
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
