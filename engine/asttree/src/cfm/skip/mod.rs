#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Skip {
    pub token: Token,
    pub args: Vec<LinkedNode>,
    pub func: Box<LinkedNode>,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl Diagnostic for Skip {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::tokens(&self.token, &self.close)
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        let mut nodes: Vec<&LinkedNode> = self.args.iter().collect();
        nodes.push(&*self.func);
        nodes
    }
}

impl<'a> Lookup<'a> for Skip {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.args
            .iter()
            .flat_map(|arg| arg.lookup_inner(self.uuid, trgs))
            .collect::<Vec<FoundNode>>()
            .into_iter()
            .chain(self.func.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for Skip {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.func
            .find_mut_by_uuid(uuid)
            .or_else(|| self.args.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for Skip {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.token, &self.close)
    }
    fn slink(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
}

impl fmt::Display for Skip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.token,
            self.open,
            self.args
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            Kind::Comma,
            self.func,
            self.close
        )
    }
}

impl From<Skip> for Node {
    fn from(val: Skip) -> Self {
        Node::ControlFlowModifier(ControlFlowModifier::Skip(val))
    }
}
