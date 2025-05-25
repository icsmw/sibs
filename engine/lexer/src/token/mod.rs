pub mod cases;
mod conflict;
mod interest;
mod keyword;
mod kind;
mod length;
mod link;
mod position;
mod read;
mod tokens;

pub(crate) use conflict::*;
pub(crate) use interest::*;
pub use keyword::*;
pub use kind::*;
pub(crate) use length::*;
pub use link::*;
pub use position::*;
pub(crate) use read::*;
pub use tokens::*;

use std::{cmp::PartialEq, fmt};
use uuid::Uuid;

/// Represents a lexical token produced by the lexer.
///
/// Each `Token` consists of a `kind`, indicating the type of token,
/// and a `pos`, representing its position in the source code.
#[derive(Debug, Clone)]
pub struct Token {
    /// Uuid of token's source. This uuid is equal to Lexer's uuid
    ///
    /// **Note:** The `src` field is intentionally excluded from `PartialEq`
    /// to simplify testing by ignoring differences in source identifiers.
    pub src: Uuid,
    /// The kind of the token.
    pub kind: Kind,
    /// The position of the token in the source code.
    pub pos: Position,
    /// Owner (Node) of token. Can be empty if parsing of Node was failed
    /// - `Uuid` - uuid of owner node
    /// - `usize` - score, which is used during setup ownership
    pub owner: Option<(Uuid, usize)>,
}

impl Token {
    pub fn is_in_position(&self, src: &Uuid, pos: usize) -> bool {
        if &self.src != src {
            return false;
        }
        self.pos.is_in(pos)
    }
    pub fn belongs(&self, src: &Uuid) -> bool {
        &self.src == src
    }
    pub fn fingerprint(&self) -> String {
        format!("{}:{}:{}", self.src, self.pos.from.abs, self.pos.to.abs)
    }
    pub fn set_owner(&mut self, uuid: &Uuid, score: usize) -> bool {
        if let Some((ow, sc)) = self.owner {
            if sc < score {
                self.owner = Some((ow, sc));
                false
            } else {
                // Rewrite owner because it sets in context of less region (score).
                self.owner = Some((*uuid, score));
                true
            }
        } else {
            self.owner = Some((*uuid, score));
            true
        }
    }
    pub fn offset(&mut self, abs: usize) {
        self.pos.from.abs += abs;
        self.pos.to.abs += abs;
    }
}

impl PartialEq for Token {
    /// Compares two `Token` instances for equality, ignoring the `src` field.
    ///
    /// This implementation checks if both `pos` and `kind` are equal.
    /// The `src` field is not considered in the comparison to facilitate testing.
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.kind == other.kind
    }
}

impl Token {
    /// Creates a new `Token` with the specified kind and position.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of the token.
    /// * `src` - Uuid of source; equal to Lexer's uuid
    /// * `from` - The starting index of the token.
    /// * `to` - The ending index of the token.
    pub fn by_pos(kind: Kind, src: &Uuid, from: TextPosition, to: TextPosition) -> Self {
        Self {
            src: src.to_owned(),
            kind,
            pos: Position::new(from, to),
            owner: None,
        }
    }

    /// Returns the identifier (`KindId`) of the token's kind.
    pub fn id(&self) -> KindId {
        self.kind.id()
    }

    #[cfg(any(test, feature = "proptests"))]
    pub fn for_test(kind: Kind) -> Self {
        Self {
            src: Uuid::new_v4(),
            kind,
            pos: Position::new(TextPosition::default(), TextPosition::default()),
            owner: None,
        }
    }
}

impl fmt::Display for Token {
    /// Formats the token as a string by displaying its kind.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
