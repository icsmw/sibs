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
}
