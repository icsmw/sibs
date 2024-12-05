pub mod error;
#[cfg(any(test, feature = "proptests"))]
mod tests;

mod token;
pub use error::E as LexerErr;
pub(crate) use error::*;
#[cfg(any(test, feature = "proptests"))]
pub use tests::*;
pub use token::*;
use uuid::Uuid;

/// The `Lexer` struct is responsible for converting input strings into tokens.
///
/// It maintains the current position in the input and provides methods
/// to read identifiers, whitespace, specific characters, and more.
#[derive(Debug)]
pub struct Lexer<'a> {
    /// The input string to be lexed.
    pub(crate) input: &'a str,
    /// The current position in the input string.
    pub(crate) pos: usize,
    /// Uuid of lexer's instance
    pub uuid: Uuid,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer` with the given input and starting offset.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the input to be lexed.
    /// * `offset` - The starting position in the input string.
    pub fn new(input: &'a str, offset: usize) -> Self {
        Lexer {
            input,
            pos: offset,
            uuid: Uuid::new_v4(),
        }
    }

    /// Reads an identifier from the current position.
    ///
    /// An identifier consists of alphanumeric characters and underscores.
    ///
    /// # Returns
    ///
    /// * A `String` containing the identifier.
    pub(crate) fn read_identifier(&mut self) -> String {
        let start = self.pos;
        while let Some(ch) = self.char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    /// Reads whitespace characters (excluding newlines) from the current position.
    ///
    /// # Returns
    ///
    /// * A `String` containing the whitespace.
    pub(crate) fn read_whitespace(&mut self) -> String {
        let start = self.pos;
        while let Some(ch) = self.char() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    /// Reads a specific number of characters from the current position.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of characters to read.
    ///
    /// # Returns
    ///
    /// * A `String` containing the read characters.
    pub(crate) fn read_nth(&mut self, count: usize) -> String {
        let start = self.pos;
        for _ in 0..count {
            if self.char().is_some() {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    /// Returns the current character without advancing the position.
    ///
    /// # Returns
    ///
    /// * `Some(char)` if there is a character at the current position.
    /// * `None` if the end of input has been reached.
    pub(crate) fn char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// Returns the remaining input from the current position.
    ///
    /// # Returns
    ///
    /// * A `String` containing the rest of the input.
    pub(crate) fn rest(&self) -> String {
        self.input[self.pos..].to_string()
    }

    /// Checks if the lexer has completed reading the input.
    ///
    /// # Returns
    ///
    /// * `true` if the end of input has been reached.
    /// * `false` otherwise.
    pub(crate) fn completed(&self) -> bool {
        self.input[self.pos..].is_empty()
    }

    /// Advances the current position by one character.
    pub(crate) fn advance(&mut self) {
        if let Some(ch) = self.char() {
            self.pos += ch.len_utf8();
        }
    }

    /// Reads characters until a specified stop character is encountered.
    ///
    /// The stop character is not included in the returned string.
    ///
    /// # Arguments
    ///
    /// * `stop` - The character at which to stop reading.
    ///
    /// # Returns
    ///
    /// * `Some(String)` containing the read characters if the stop character is found.
    /// * `None` if the stop character is not found before the end of input.
    pub(crate) fn read_until(&mut self, stop: char) -> Option<String> {
        let mut str = String::new();
        let mut escaped = false;
        while let Some(ch) = self.char() {
            if ch == stop && !escaped {
                return Some(str);
            }
            escaped = ch == '\\';
            str.push(ch);
            self.advance();
        }
        None
    }

    /// Reads the rest of the input and advances the position to the end.
    ///
    /// # Returns
    ///
    /// * A `String` containing the rest of the input.
    pub(crate) fn read_to_end(&mut self) -> String {
        let rest = self.rest();
        self.pos = self.input.len();
        rest
    }

    /// Creates a checkpoint of the current position in the lexer.
    ///
    /// Returns a closure that can restore the lexer's position to the checkpoint.
    ///
    /// # Returns
    ///
    /// * A closure that takes a mutable reference to a `Lexer` and restores its position.
    pub(crate) fn pin(&mut self) -> impl Fn(&mut Lexer) -> usize {
        let pos = self.pos;
        move |lexer: &mut Lexer| {
            let to_restore = lexer.pos;
            lexer.pos = pos;
            to_restore
        }
    }

    /// Reads tokens from the input and returns a `Tokens` collection.
    ///
    /// If `new_file` is `true`, it inserts a `BOF` (beginning of file) token at the start
    /// and an `EOF` (end of file) token at the end.
    ///
    /// # Arguments
    ///
    /// * `new_file` - Indicates whether to treat the input as a new file.
    ///
    /// # Returns
    ///
    /// * `Ok(Tokens)` containing the parsed tokens.
    /// * `Err(E)` if an error occurs during lexing.
    pub fn read(&mut self, new_file: bool) -> Result<Tokens, E> {
        let mut tokens = if new_file {
            Tokens::with(vec![Token::by_pos(Kind::BOF, &self.uuid, 0, 0)])
        } else {
            Tokens::default()
        };
        while let Some(tk) = Token::read(self, &tokens)? {
            tokens.add(tk);
        }
        if !self.completed() {
            Err(E::FailRecognizeContent(self.pos))
        } else {
            if new_file {
                tokens.add(Token::by_pos(Kind::EOF, &self.uuid, self.pos, self.pos));
            }
            Ok(tokens)
        }
    }
}
