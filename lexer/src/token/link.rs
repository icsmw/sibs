use crate::*;
use uuid::Uuid;

/// Represents the position of a content in the source code.
///
/// The `SrcLink` struct holds the starting and ending indices,
/// allowing for precise location tracking within the source code.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct SrcLink {
    /// The position of node as itself.
    pub pos: Position,
    /// The position, including ppm
    pub expos: Position,
    /// The uuid of source code file
    pub src: Uuid,
}

impl SrcLink {
    /// Creates a new `SrcLink` with the specified starting and ending indices.
    ///
    /// # Arguments
    ///
    /// * `src` - The uuid of source code file
    pub fn new(src: &Uuid) -> Self {
        Self {
            pos: Position::default(),
            expos: Position::default(),
            src: *src,
        }
    }

    pub fn from(&self) -> usize {
        self.pos.from
    }

    pub fn to(&self) -> usize {
        self.pos.to
    }

    pub fn exfrom(&self) -> usize {
        self.expos.from
    }

    pub fn exto(&self) -> usize {
        self.expos.to
    }

    pub fn set_pos(&mut self, from: &Token, to: &Token) {
        self.pos = Position::new(from.pos.from, to.pos.to);
    }

    pub fn set_expos(&mut self, from: &Token, to: &Token) {
        self.expos = Position::new(from.pos.from, to.pos.to);
    }
}
