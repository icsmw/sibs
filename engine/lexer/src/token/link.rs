use crate::*;
use uuid::Uuid;

/// Represents the position of a content in the source code.
///
/// The `SrcLink` struct holds the starting and ending indices,
/// allowing for precise location tracking within the source code.
///
/// Note. Not `pos`, not `expos` doesn't include metadata (meta or
/// comments). `pos` - includes node at itself without ppm; `expos` -
/// includes node and its ppm.
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

    pub fn from_tk(token: &Token) -> Self {
        Self {
            pos: token.pos.clone(),
            expos: token.pos.clone(),
            src: token.src,
        }
    }

    pub fn from_tks(from: &Token, to: &Token) -> Self {
        Self {
            pos: Position {
                from: from.pos.from,
                to: to.pos.to,
            },
            expos: Position {
                from: from.pos.from,
                to: to.pos.to,
            },
            src: from.src,
        }
    }

    pub fn belongs(&self, src: &Uuid) -> bool {
        &self.src == src
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
        self.src = from.src;
    }

    pub fn set_expos(&mut self, from: &Token, to: &Token) {
        self.expos = Position::new(from.pos.from, to.pos.to);
        self.src = from.src;
    }
}
