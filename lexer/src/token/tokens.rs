use crate::*;

/// A collection of tokens produced by the lexer.
///
/// The `Tokens` struct holds a vector of `Token` instances,
/// providing methods to manipulate and inspect the token stream.
#[derive(Debug, Default)]
pub struct Tokens {
    /// The vector of tokens.
    pub tokens: Vec<Token>,
}

impl Tokens {
    /// Creates a new `Tokens` instance with the given vector of tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A vector of `Token` instances to initialize the `Tokens` struct.
    pub fn with(tokens: Vec<Token>) -> Self {
        Tokens { tokens }
    }

    /// Adds a token to the collection.
    ///
    /// # Arguments
    ///
    /// * `token` - The `Token` instance to add.
    pub fn add(&mut self, token: Token) {
        self.tokens.push(token);
    }

    /// Checks if the last significant token indicates a new line.
    ///
    /// This method iterates over the tokens in reverse, skipping any whitespace,
    /// to determine if the last significant token is a line break or the beginning of the file.
    ///
    /// # Returns
    ///
    /// * `true` if the last significant token is a new line or beginning of file.
    /// * `false` otherwise.
    pub fn is_nl(&self) -> bool {
        let mut nl = false;
        for tk in self.tokens.iter().rev() {
            if matches!(tk.id(), KindId::Whitespace) {
                continue;
            } else if matches!(
                tk.id(),
                KindId::LF | KindId::CR | KindId::CRLF | KindId::BOF
            ) {
                nl = true;
                break;
            } else {
                break;
            }
        }
        nl
    }

    /// Returns a reference to the last token in the collection.
    ///
    /// # Returns
    ///
    /// * `Some(&Token)` if the collection is not empty.
    /// * `None` if the collection is empty.
    pub fn last(&self) -> Option<&Token> {
        self.tokens.last()
    }

    /// Returns the number of tokens in the collection.
    ///
    /// # Returns
    ///
    /// * The number of tokens as a `usize`.
    pub fn count(&self) -> usize {
        self.tokens.len()
    }

    /// Returns an iterator over the tokens in the collection.
    ///
    /// # Returns
    ///
    /// * An iterator of type `std::slice::Iter<'_, Token>`.
    pub fn iter(&self) -> std::slice::Iter<'_, Token> {
        self.tokens.iter()
    }
}
