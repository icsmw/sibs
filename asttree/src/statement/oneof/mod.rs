#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct OneOf {
    pub commands: Vec<LinkedNode>,
    pub token: Token,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl fmt::Display for OneOf {
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
