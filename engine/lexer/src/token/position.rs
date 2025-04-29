use crate::*;

/// Represents the position of a token or element in the source code.
///
/// The `Position` struct holds the starting and ending indices,
/// allowing for precise location tracking within the source code.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Position {
    /// The starting index (inclusive).
    pub from: usize,
    /// The ending index (exclusive).
    pub to: usize,
}

impl Position {
    /// Creates a new `Position` with the specified starting and ending indices.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting index.
    /// * `to` - The ending index.
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
    pub fn tokens(from: &Token, to: &Token) -> Self {
        Self {
            from: from.pos.from,
            to: to.pos.to,
        }
    }
    pub fn is_in(&self, pos: usize) -> bool {
        pos >= self.from && pos <= self.to
    }
}

/// Represents the position of a token or element in the source code.
/// Includes uuid (`src`) of source.
///
/// The `Position` struct holds the starting and ending indices,
/// allowing for precise location tracking within the source code.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct LinkedPosition {
    /// The starting index (inclusive).
    pub from: usize,
    /// The ending index (exclusive).
    pub to: usize,
    /// The uuid of source code file
    pub src: Uuid,
}

impl LinkedPosition {
    /// Creates a new `Position` with the specified starting and ending indices.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting index.
    /// * `to` - The ending index.
    pub fn new(from: usize, to: usize, src: &Uuid) -> Self {
        Self {
            from,
            to,
            src: *src,
        }
    }
}

impl From<&Token> for LinkedPosition {
    fn from(token: &Token) -> Self {
        Self {
            from: token.pos.from,
            to: token.pos.to,
            src: token.src.to_owned(),
        }
    }
}

impl From<(&Token, &Token)> for LinkedPosition {
    fn from((from, to): (&Token, &Token)) -> Self {
        Self {
            from: from.pos.from,
            to: to.pos.to,
            src: from.src.to_owned(),
        }
    }
}

impl From<&SrcLink> for LinkedPosition {
    fn from(link: &SrcLink) -> Self {
        Self {
            from: link.pos.from,
            to: link.pos.to,
            src: link.src,
        }
    }
}
