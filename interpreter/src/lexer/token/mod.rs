mod conflict;
mod interest;
mod intstring;
mod kind;
mod length;
mod position;
mod read;
#[cfg(test)]
mod tests;
mod tokens;

pub use conflict::*;
pub use interest::*;
pub use intstring::*;
pub use kind::*;
pub use length::*;
pub use position::*;
pub use read::*;
pub use tokens::*;

use std::fmt;
#[cfg(test)]
pub use tests::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: Kind,
    pub pos: Position,
}

impl Token {
    pub fn new(kind: Kind, pos: Position) -> Self {
        Self { kind, pos }
    }
    pub fn by_pos(kind: Kind, from: usize, to: usize) -> Self {
        Self {
            kind,
            pos: Position::new(from, to),
        }
    }
    pub fn id(&self) -> KindId {
        self.kind.id()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
