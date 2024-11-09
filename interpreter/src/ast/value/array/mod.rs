#[cfg(test)]
mod proptests;
mod read;

use crate::*;
use lexer::Kind;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Array {
    els: Vec<Node>,
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            Kind::LeftBracket,
            self.els
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            Kind::RightBracket
        )
    }
}
