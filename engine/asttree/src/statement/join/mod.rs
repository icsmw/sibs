#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Join {
    pub commands: Vec<LinkedNode>,
    pub token: Token,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for Join {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.commands
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for Join {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.commands.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for Join {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.token, &self.close)
    }
    fn slink(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.token,
            self.open,
            self.commands
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(&Kind::Comma.to_string()),
            self.close
        )
    }
}

impl From<Join> for Node {
    fn from(val: Join) -> Self {
        Node::Statement(Statement::Join(val))
    }
}
