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

impl TaskCall {
    pub fn get_name(&self) -> String {
        self.reference
            .iter()
            .map(|(part, _)| part.clone())
            .collect::<Vec<String>>()
            .join(":")
    }
    pub fn get_src(&self) -> Option<SrcLink> {
        if let Some((_, ft)) = self.reference.first() {
            let pos = Position {
                from: ft.pos.from,
                to: self.close.pos.to,
            };
            Some(SrcLink {
                pos: pos.clone(),
                expos: pos,
                src: ft.src,
            })
        } else {
            None
        }
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
