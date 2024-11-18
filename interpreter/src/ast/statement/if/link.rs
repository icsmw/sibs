use lexer::SrcLink;

use crate::*;

impl From<&IfCase> for SrcLink {
    fn from(node: &IfCase) -> Self {
        match node {
            IfCase::If(_, block, tk) => {
                let block: SrcLink = block.into();
                SrcLink {
                    from: tk.pos.from,
                    to: block.to,
                    src: tk.src,
                }
            }
            IfCase::Else(block, tk) => {
                let block: SrcLink = block.into();
                SrcLink {
                    from: tk.pos.from,
                    to: block.to,
                    src: tk.src,
                }
            }
        }
    }
}

impl From<&If> for SrcLink {
    fn from(node: &If) -> Self {
        if let (Some(f), Some(l)) = (node.cases.first(), node.cases.last()) {
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
