mod conflict;
mod interest;
mod intstring;
mod kind;
mod length;
mod position;
mod read;
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

/// Represents a lexical token produced by the lexer.
///
/// Each `Token` consists of a `kind`, indicating the type of token,
/// and a `pos`, representing its position in the source code.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// The kind of the token.
    pub kind: Kind,
    /// The position of the token in the source code.
    pub pos: Position,
}

impl Token {
    /// Creates a new `Token` with the specified kind and position.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of the token.
    /// * `from` - The starting index of the token.
    /// * `to` - The ending index of the token.
    pub fn by_pos(kind: Kind, from: usize, to: usize) -> Self {
        Self {
            kind,
            pos: Position::new(from, to),
        }
    }

    /// Returns the identifier (`KindId`) of the token's kind.
    pub fn id(&self) -> KindId {
        self.kind.id()
    }
}

impl fmt::Display for Token {
    /// Formats the token as a string by displaying its kind.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
