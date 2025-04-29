#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum CommandPart {
    Open(Token),
    Literal(Token),
    Expression(LinkedNode),
    Close(Token),
}

impl Diagnostic for CommandPart {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        match self {
            CommandPart::Open(tk) | CommandPart::Close(tk) | CommandPart::Literal(tk, ..) => {
                if !tk.belongs(src) {
                    false
                } else {
                    self.get_position().is_in(pos)
                }
            }
            CommandPart::Expression(n) => {
                if !n.md.link.belongs(src) {
                    false
                } else {
                    self.get_position().is_in(pos)
                }
            }
        }
    }
    fn get_position(&self) -> Position {
        match self {
            CommandPart::Open(tk) | CommandPart::Close(tk) | CommandPart::Literal(tk, ..) => {
                tk.pos.clone()
            }
            CommandPart::Expression(n) => n.md.link.pos.clone(),
        }
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        match self {
            CommandPart::Open(..) | CommandPart::Close(..) | CommandPart::Literal(..) => Vec::new(),
            CommandPart::Expression(n) => vec![n],
        }
    }
}

impl<'a> LookupInner<'a> for &'a CommandPart {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        match self {
            CommandPart::Open(..) | CommandPart::Close(..) | CommandPart::Literal(..) => Vec::new(),
            CommandPart::Expression(n) => n.lookup_inner(owner, trgs),
        }
    }
}

impl FindMutByUuid for CommandPart {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        match self {
            CommandPart::Open(..) | CommandPart::Close(..) | CommandPart::Literal(..) => None,
            CommandPart::Expression(n) => n.find_mut_by_uuid(uuid),
        }
    }
}

impl fmt::Display for CommandPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Open(tk) => tk.to_string(),
                Self::Close(tk) => tk.to_string(),
                Self::Literal(tk) => tk.to_string(),
                Self::Expression(n) => format!("{} {n} {}", Kind::LeftBrace, Kind::RightBrace),
            }
        )
    }
}

impl FindMutByUuid for Vec<CommandPart> {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.iter_mut().find_map(|n| n.find_mut_by_uuid(uuid))
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub nodes: Vec<CommandPart>,
    pub token: Token,
    pub uuid: Uuid,
}

impl Diagnostic for Command {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.token.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        if let (Some(first), Some(last)) = (self.nodes.first(), self.nodes.last()) {
            Position::new(first.get_position().from, last.get_position().to)
        } else {
            self.token.pos.clone()
        }
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        self.nodes.iter().flat_map(|n| n.childs()).collect()
    }
}

impl<'a> Lookup<'a> for Command {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.nodes
            .iter()
            .flat_map(|n| n.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for Command {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.nodes.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for Command {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl From<Command> for Node {
    fn from(val: Command) -> Self {
        Node::Expression(Expression::Command(val))
    }
}
