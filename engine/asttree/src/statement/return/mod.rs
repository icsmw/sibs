#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Return {
    pub token: Token,
    pub node: Option<Box<LinkedNode>>,
    pub uuid: Uuid,
    pub targets: Vec<Uuid>,
}

impl Return {
    pub fn add_target(&mut self, uuid: &Uuid) {
        if !self.targets.contains(uuid) {
            self.targets.push(*uuid);
        }
    }
    pub fn is_target_included(&self, uuid: &Uuid) -> bool {
        self.targets.contains(uuid)
    }
    pub fn is_assigned(&self) -> bool {
        !self.targets.is_empty()
    }
}

impl Diagnostic for Return {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        self.token.pos.clone()
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        self.node
            .as_ref()
            .map(|n| vec![&**n])
            .unwrap_or_else(|| Vec::new())
    }
}

impl<'a> Lookup<'a> for Return {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node.as_ref().lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for Return {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.node.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for Return {
    fn link(&self) -> SrcLink {
        if let Some(n) = self.node.as_ref() {
            src_from::tk_and_node(&self.token, n)
        } else {
            src_from::tk(&self.token)
        }
    }
    fn slink(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.token,
            self.node
                .as_ref()
                .map(|n| format!(" {n}"))
                .unwrap_or_default()
        )
    }
}

impl From<Return> for Node {
    fn from(val: Return) -> Self {
        Node::Statement(Statement::Return(val))
    }
}
