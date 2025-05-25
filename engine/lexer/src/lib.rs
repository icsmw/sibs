pub mod error;
#[cfg(any(test, feature = "proptests"))]
mod tests;

mod token;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub use error::E as LexerError;
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
    /// The current line number
    pub(crate) ln: usize,
    /// The current offset in current like
    pub(crate) column: usize,
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
            ln: 0,
            column: 0,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn inherit(&self, input: &'a str) -> Self {
        Lexer {
            input,
            pos: 0,
            ln: 0,
            column: 0,
            uuid: self.uuid,
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
            if ch == '\n' && ch == '\r' {
                self.ln += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
            self.pos += ch.len_utf8();
        }
    }

    /// Checks next char
    ///
    /// # Arguments
    ///
    /// * `ch` - char expectation.
    ///
    /// # Returns
    ///
    /// * `true` next char fits to expectation
    /// * `false` next char doesn't fit to expectation
    pub(crate) fn is_next(&mut self, ch: char) -> bool {
        let pin = self.pin();
        self.advance();
        let result = self.char().map(|nch| nch == ch).unwrap_or(false);
        pin(self);
        result
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
    pub(crate) fn read_until(&mut self, stop: &[char]) -> Option<(String, char)> {
        let mut str = String::new();
        let mut escaped = false;
        while let Some(ch) = self.char() {
            if stop.contains(&ch) && !escaped {
                return Some((str, ch));
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
        let ln = self.ln;
        let column = self.column;
        move |lexer: &mut Lexer| {
            let to_restore = lexer.pos;
            lexer.pos = pos;
            lexer.ln = ln;
            lexer.column = column;
            to_restore
        }
    }

    /// Reads tokens from the input and returns a `Tokens` collection.
    ///
    /// If `new_file` is `true`, it inserts a `BOF` (beginning of file) token at the start
    /// and an `EOF` (end of file) token at the end.
    ///
    /// # Returns
    ///
    /// * `Ok(Tokens)` containing the parsed tokens.
    /// * `Err(E)` if an error occurs during lexing.
    pub fn read(&mut self) -> Result<Tokens, E> {
        let mut tokens = Tokens::with(vec![Token::by_pos(Kind::BOF, &self.uuid, 0, 0)]);
        while let Some(tk) = Token::read(self, &tokens)? {
            if let Some(tks) = cases::check(self, &tk)? {
                tokens.extend(tks);
            } else {
                tokens.add(tk);
            }
        }
        if !self.completed() {
            Err(E::FailRecognizeContent(self.pos))
        } else {
            tokens.add(Token::by_pos(Kind::EOF, &self.uuid, self.pos, self.pos));
            Ok(tokens)
        }
    }
}

pub struct BoundLexer {
    pub filename: PathBuf,
    pub cwd: PathBuf,
    pub tokens: Vec<Token>,
    pub uuid: Uuid,
}

impl BoundLexer {
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, E> {
        let filename = filename.as_ref().to_path_buf();
        if !filename.exists() {
            return Err(E::FileNotFound(filename));
        }
        let Some(cwd) = filename.parent().map(|cwd| cwd.to_path_buf()) else {
            return Err(E::NoCwdFolder(filename));
        };
        let content =
            fs::read_to_string(&filename).map_err(|e| E::FailToReadFile(filename.clone(), e))?;
        let mut lexer = Lexer::new(&content, 0);
        let tokens = lexer.read()?.tokens;
        Ok(Self {
            filename,
            cwd,
            tokens,
            uuid: lexer.uuid,
        })
    }
    pub fn inner(self) -> (PathBuf, PathBuf, Vec<Token>, Uuid) {
        (self.filename, self.cwd, self.tokens, self.uuid)
    }
}
