#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TaskCall {
    pub args: Vec<LinkedNode>,
    pub reference: Vec<(String, Token)>,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl Diagnostic for TaskCall {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.open.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(
            self.reference
                .first()
                .map(|(_, tk)| tk.pos.from)
                .unwrap_or(self.open.pos.from),
            self.close.pos.to,
        )
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        self.args.iter().collect()
    }
}

impl TaskCall {
    pub fn get_name(&self) -> String {
        self.reference
            .iter()
            .map(|(part, _)| part.clone())
            .collect::<Vec<String>>()
            .join(":")
    }
}

impl<'a> Lookup<'a> for TaskCall {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.args
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for TaskCall {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.args.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for TaskCall {
    fn link(&self) -> SrcLink {
        if let Some((_, open)) = self.reference.first() {
            src_from::tks(open, &self.close)
        } else {
            src_from::tks(&self.open, &self.close)
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for TaskCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            Kind::Colon,
            self.reference
                .iter()
                .map(|(s, _)| s.to_owned())
                .collect::<Vec<String>>()
                .join(&Kind::Colon.to_string()),
            Kind::LeftParen,
            self.args
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            Kind::RightParen
        )
    }
}

impl From<TaskCall> for Node {
    fn from(val: TaskCall) -> Self {
        Node::Expression(Expression::TaskCall(val))
    }
}
