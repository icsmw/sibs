mod chars;
mod entry;
mod error;
mod reader;
mod words;

use error::E;
pub use reader::{Mapper, Reader};

pub struct Parser {
    content: String,
}

impl Parser {
    fn next(&self) -> Result<(), E> {
        let mut str: String = String::new();
        let mut pass: usize = 0;
        for char in self.content.chars() {
            pass += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if char.is_ascii_digit() && str.is_empty() {
                // return Err(ENextErr::NumericFirst());
            }
            if char.is_ascii_whitespace() && str.is_empty() {
                continue;
            }
        }
        Ok(())
    }
}
