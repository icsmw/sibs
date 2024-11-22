use crate::*;
use asttree::*;

impl From<&Component> for SrcLink {
    fn from(node: &Component) -> Self {
        SrcLink {
            from: node.sig.pos.from,
            to: node.close_bl.pos.to,
            src: node.close_bl.src,
        }
    }
}
