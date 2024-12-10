#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ComparisonGroup {
    pub open: Token,
    pub close: Token,
    pub node: Box<LinkedNode>,
    pub negation: Option<Token>,
    pub uuid: Uuid,
}

impl fmt::Display for ComparisonGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} {} {}",
            self.negation
                .as_ref()
                .map(|tk| format!("{tk} "))
                .unwrap_or_default(),
            self.open,
            self.node,
            self.close
        )
    }
}

impl From<ComparisonGroup> for Node {
    fn from(val: ComparisonGroup) -> Self {
        Node::Expression(Expression::ComparisonGroup(val))
    }
}
