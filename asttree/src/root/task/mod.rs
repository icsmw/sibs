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
    pub block: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl Task {
    pub fn get_name(&self) -> String {
        self.name.to_string()
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
