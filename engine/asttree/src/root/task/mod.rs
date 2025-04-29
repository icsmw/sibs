#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Task {
    pub vis: Option<Token>,
    pub sig: Token,
    pub name: Token,
    pub open: Token,
    pub close: Token,
    pub args: Vec<LinkedNode>,
    // GateKeepers
    pub gts: Vec<LinkedNode>,
    pub block: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl Task {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl Diagnostic for Task {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.sig.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(
            self.vis
                .as_ref()
                .map(|tk| tk.pos.from)
                .unwrap_or(self.sig.pos.from),
            self.block.md.link.to(),
        )
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        let mut nodes: Vec<&LinkedNode> = [
            self.args.iter().collect::<Vec<&LinkedNode>>(),
            self.gts.iter().collect::<Vec<&LinkedNode>>(),
        ]
        .concat();
        nodes.push(&*self.block);
        nodes
    }
}

impl<'a> Lookup<'a> for Task {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.args
            .iter()
            .flat_map(|arg| arg.lookup_inner(self.uuid, trgs))
            .collect::<Vec<FoundNode>>()
            .into_iter()
            .chain(self.block.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for Task {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.block
            .find_mut_by_uuid(uuid)
            .or_else(|| self.args.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for Task {
    fn link(&self) -> SrcLink {
        src_from::tk_and_node(&self.sig, &self.block)
    }
    fn slink(&self) -> SrcLink {
        src_from::tks(&self.sig, &self.close)
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} {} {} {} {} {}",
            self.vis
                .as_ref()
                .map(|vis| format!("{vis} "))
                .unwrap_or_default(),
            self.sig,
            self.name,
            self.open,
            self.args
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            self.close,
            self.block
        )
    }
}

impl From<Task> for Node {
    fn from(val: Task) -> Self {
        Node::Root(Root::Task(val))
    }
}
