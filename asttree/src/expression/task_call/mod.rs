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
