mod ast;
mod token;

use uuid::Uuid;

/// Represents the position of a content in the source code.
///
/// The `SrcLink` struct holds the starting and ending indices,
/// allowing for precise location tracking within the source code.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct SrcLink {
    /// The starting index (inclusive).
    pub from: usize,
    /// The ending index (exclusive).
    pub to: usize,
    /// The uuid of source code file
    pub src: Uuid,
}

impl SrcLink {
    /// Creates a new `SrcLink` with the specified starting and ending indices.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting index.
    /// * `to` - The ending index.
    /// * `src` - The uuid of source code file
    pub fn new(from: usize, to: usize, src: &Uuid) -> Self {
        Self {
            from,
            to,
            src: *src,
        }
    }
}
