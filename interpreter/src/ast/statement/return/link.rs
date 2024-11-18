use lexer::SrcLink;

use crate::*;

impl From<&Return> for SrcLink {
    fn from(node: &Return) -> Self {
        node.node
            .as_ref()
            .map(|n| {
                let val: SrcLink = n.into();
                SrcLink {
                    from: node.token.pos.from,
                    to: val.to,
                    src: node.token.src,
                }
            })
            .unwrap_or((&node.token).into())
    }
}
