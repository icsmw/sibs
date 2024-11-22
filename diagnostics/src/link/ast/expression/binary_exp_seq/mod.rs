use crate::*;
use asttree::*;

impl From<&BinaryExpSeq> for SrcLink {
    fn from(node: &BinaryExpSeq) -> Self {
        if let (Some(f), Some(l)) = (node.nodes.first(), node.nodes.last()) {
            let f: SrcLink = f.into();
            let l: SrcLink = l.into();
            SrcLink {
                from: f.from,
                to: l.to,
                src: f.src,
            }
        } else {
            SrcLink::default()
        }
    }
}
