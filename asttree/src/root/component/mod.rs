#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Component {
    pub sig: Token,
    pub name: Token,
    pub path: String,
    pub nodes: Vec<LinkedNode>,
    pub open_bl: Token,
    pub close_bl: Token,
    pub uuid: Uuid,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {}",
            self.sig,
            self.name,
            Kind::LeftParen,
            self.path,
            Kind::RightParen,
            self.open_bl,
            self.nodes
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            self.close_bl
        )
    }
}
