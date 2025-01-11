use crate::*;

pub(crate) mod src_from {
    use crate::*;

    pub fn nodes(from: &LinkedNode, to: &LinkedNode) -> SrcLink {
        SrcLink {
            pos: Position {
                from: from.md.link.from(),
                to: to.md.link.to(),
            },
            expos: Position {
                from: from.md.link.from(),
                to: to.md.link.to(),
            },
            src: from.md.link.src,
        }
    }
    pub fn tk_and_node(from: &Token, to: &LinkedNode) -> SrcLink {
        SrcLink {
            pos: Position {
                from: from.pos.from,
                to: to.md.link.to(),
            },
            expos: Position {
                from: from.pos.from,
                to: to.md.link.to(),
            },
            src: from.src,
        }
    }
    pub fn tk(tk: &Token) -> SrcLink {
        SrcLink::from_tk(tk)
    }
    pub fn tks(from: &Token, to: &Token) -> SrcLink {
        SrcLink::from_tks(from, to)
    }
}

pub trait SrcLinking {
    fn link(&self) -> SrcLink;
    fn slink(&self) -> SrcLink;
}
