use crate::*;
use asttree::*;

impl From<&Task> for SrcLink {
    fn from(node: &Task) -> Self {
        let block: SrcLink = node.block.as_ref().into();
        if let Some(vis) = node.vis.as_ref() {
            SrcLink {
                from: vis.pos.from,
                to: block.to,
                src: block.src,
            }
        } else {
            SrcLink {
                from: node.sig.pos.from,
                to: block.to,
                src: block.src,
            }
        }
    }
}
