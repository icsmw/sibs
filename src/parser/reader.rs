use std::collections::HashMap;
use uuid::Uuid;

use crate::parser::{chars, E};

#[derive(Debug)]
pub struct Reader {
    pub content: String,
    pub pos: usize,
    pub map: HashMap<Uuid, (usize, usize)>,
}

impl Reader {
    pub fn new(content: String) -> Self {
        Self {
            content,
            pos: 0,
            map: HashMap::new(),
        }
    }
    pub fn rest(&self) -> &str {
        &self.content[self.pos..]
    }
    pub fn to_end(&mut self) -> String {
        let rest = self.rest().to_string();
        self.pos = if !self.content.is_empty() {
            self.content.len()
        } else {
            0
        };
        rest
    }
    pub fn move_to_char(&mut self, target: char) -> Result<bool, E> {
        let content = &self.content[self.pos..];
        let mut pos: usize = 0;
        for char in content.chars() {
            pos += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if char.is_ascii_whitespace() {
                continue;
            }
            return if char == target {
                self.pos += pos;
                Ok(true)
            } else {
                Ok(false)
            };
        }
        Ok(false)
    }

    pub fn read_letters(
        &mut self,
        stop_on: &[char],
        stay_on_stop_char: bool,
    ) -> Result<Option<(String, char)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let content = &self.content[self.pos..];
        for char in content.chars() {
            pos += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if char.is_ascii_whitespace() || stop_on.contains(&char) {
                return if str.is_empty() {
                    Ok(None)
                } else {
                    self.pos += pos - if stay_on_stop_char { 0 } else { 1 };
                    Ok(Some((str, char)))
                };
            }
            if !char.is_alphabetic() {
                Err(E::UnexpectedChar(char))?;
            }
            str.push(char);
        }
        Ok(None)
    }

    pub fn read_letters_to_end(
        &mut self,
        stop_on: &[char],
        stay_on_stop_char: bool,
    ) -> Result<Option<(String, Option<char>)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let content = &self.content[self.pos..];
        for char in content.chars() {
            pos += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if char.is_ascii_whitespace() || stop_on.contains(&char) {
                return if str.is_empty() {
                    Ok(None)
                } else {
                    self.pos += pos - if stay_on_stop_char { 0 } else { 1 };
                    Ok(Some((str, Some(char))))
                };
            }
            if !char.is_alphabetic() {
                Err(E::UnexpectedChar(char))?;
            }
            str.push(char);
        }
        Ok(Some((str, None)))
    }

    pub fn read_word(
        &mut self,
        stop_on: &[char],
        stay_on_stop_char: bool,
    ) -> Result<Option<(String, char)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let content = &self.content[self.pos..];
        for char in content.chars() {
            pos += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if stop_on.contains(&char) {
                return if str.is_empty() {
                    Ok(None)
                } else {
                    self.pos += pos - if stay_on_stop_char { 0 } else { 1 };
                    Ok(Some((str, char)))
                };
            }
            if !char.is_alphabetic() {
                Err(E::UnexpectedChar(char))?;
            }
            str.push(char);
        }
        Ok(None)
    }

    pub fn read_until(
        &mut self,
        stop_on: &[char],
        cursor_after_stop_char: bool,
    ) -> Result<Option<(String, char)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.content[self.pos..];
        for char in content.chars() {
            pos += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }

            if !serialized && stop_on.contains(&char) {
                return if str.is_empty() {
                    Ok(None)
                } else {
                    self.pos += pos - if cursor_after_stop_char { 0 } else { 1 };
                    Ok(Some((str, char)))
                };
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        Ok(None)
    }

    pub fn read_until_wt(
        &mut self,
        cursor_after_stop_char: bool,
    ) -> Result<Option<(String, char)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.content[self.pos..];
        for char in content.chars() {
            pos += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if !serialized && char.is_whitespace() {
                return if str.is_empty() {
                    Ok(None)
                } else {
                    self.pos += pos - if cursor_after_stop_char { 0 } else { 1 };
                    Ok(Some((str, char)))
                };
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        Ok(None)
    }

    pub fn unserialize(content: &str) -> String {
        let mut str: String = String::new();
        for char in content.chars() {
            if char != chars::SERIALIZING {
                str.push(char);
            }
        }
        str.trim().to_string()
    }
}
