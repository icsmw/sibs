pub mod chars;
pub mod entry;
pub mod words;
use crate::{
    context::Context,
    error::E,
    functions::{reader::import::Import, Implementation},
};
use entry::{Component, Function, Reading};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug)]
pub struct Mapper {
    pub map: HashMap<usize, (usize, usize)>,
    pub index: usize,
}

impl Mapper {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            index: 0,
        }
    }
    fn add(&mut self, pos: (usize, usize)) -> usize {
        self.index += 1;
        self.map.insert(self.index, pos);
        self.index
    }
}
#[derive(Debug)]
pub struct Reader<'a> {
    pub content: String,
    pub pos: usize,
    pub mapper: &'a mut Mapper,
    pub offset: usize,
    holded: Option<usize>,
}

impl<'a> Reader<'a> {
    pub fn new(content: String, mapper: &'a mut Mapper, offset: usize) -> Self {
        Self {
            content,
            pos: 0,
            mapper,
            offset,
            holded: None,
        }
    }
    pub fn hold(&mut self) {
        self.holded = Some(self.pos);
    }
    pub fn roll_back(&mut self) {
        if let Some(pos) = self.holded.take() {
            self.pos = pos;
        }
    }
    pub fn inherit(&mut self, content: String) -> Reader<'_> {
        let len = content.len();
        Reader::new(
            content,
            self.mapper,
            if self.pos < len { 0 } else { self.pos - len },
        )
    }
    pub fn get_index_until_current(&mut self, from: usize) -> usize {
        self.mapper.add((from, self.pos))
    }
    pub fn rest(&self) -> &str {
        &self.content[self.pos..]
    }
    pub fn to_end(&mut self) -> (usize, String) {
        let rest = self.rest().to_string();
        let start = self.pos;
        self.pos = if !self.content.is_empty() {
            self.content.len()
        } else {
            0
        };
        (self.add_to_map((start, self.pos)), rest)
    }
    pub fn next_char(&self) -> Option<char> {
        self.content[self.pos..].chars().next()
    }
    pub fn move_to_char(&mut self, targets: &[char]) -> Result<Option<char>, E> {
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
            return if targets.contains(&char) {
                self.pos += pos;
                Ok(Some(char))
            } else {
                Ok(None)
            };
        }
        Ok(None)
    }
    pub fn move_to_word(&mut self, targets: &[&str]) -> Result<Option<String>, E> {
        let only_alphabetic = !targets.join("").chars().any(|c| !c.is_alphabetic());
        let content = &self.content[self.pos..];
        let mut pos: usize = 0;
        let mut str = String::new();
        for char in content.chars() {
            pos += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if char.is_ascii_whitespace() && str.is_empty() {
                continue;
            } else if (char.is_ascii_whitespace() || (only_alphabetic && !char.is_alphabetic()))
                && !str.is_empty()
            {
                return if targets.contains(&str.as_str()) {
                    self.pos += pos - 1;
                    Ok(Some(str))
                } else {
                    Ok(None)
                };
            }
            str.push(char);
        }
        Ok(None)
    }
    pub fn stop_on_char(&mut self, target: char, cancel_on: &[char]) -> Result<bool, E> {
        let content = &self.content[self.pos..];
        let mut pos: usize = 0;
        let mut serialized: bool = false;
        for char in content.chars() {
            pos += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if serialized || char == chars::SERIALIZING {
                serialized = char == chars::SERIALIZING;
                continue;
            }
            serialized = char == chars::SERIALIZING;
            if cancel_on.contains(&char) {
                return Ok(false);
            }
            if char == target {
                self.pos += pos;
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub fn has_char(&mut self, target: char) -> Result<bool, E> {
        let content = &self.content[self.pos..];
        let mut serialized: bool = false;
        for char in content.chars() {
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if serialized || char == chars::SERIALIZING {
                serialized = char == chars::SERIALIZING;
                continue;
            }
            serialized = char == chars::SERIALIZING;
            if char == target {
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub fn has_word(&mut self, targets: &[&str]) -> Result<bool, E> {
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.content[self.pos..];
        for char in content.chars() {
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if !serialized && char != chars::SERIALIZING {
                str.push(char);
            }
            serialized = char == chars::SERIALIZING;
            for word in targets.iter() {
                if str.ends_with(word) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    pub fn read_letters(
        &mut self,
        stop_on: &[char],
        allowed: &[char],
        cursor_after_stop_char: bool,
    ) -> Result<Option<(String, Option<char>, usize)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let start = self.pos;
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
                    self.pos += pos - if cursor_after_stop_char { 0 } else { 1 };
                    Ok(Some((str, Some(char), self.add_to_map((start, self.pos)))))
                };
            }
            if !char.is_alphabetic() && !allowed.contains(&char) {
                Err(E::UnexpectedChar(char))?;
            }
            str.push(char);
        }
        if !str.is_empty() {
            self.pos += pos - if cursor_after_stop_char { 0 } else { 1 };
            Ok(Some((str, None, self.add_to_map((start, self.pos)))))
        } else {
            Ok(None)
        }
    }
    pub fn read_word(
        &mut self,
        stop_on: &[char],
        stay_on_stop_char: bool,
    ) -> Result<Option<(String, char, usize)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let start = self.pos;
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
                    Ok(Some((str, char, self.add_to_map((start, self.pos)))))
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
        to_end: bool,
    ) -> Result<Option<(String, char, usize)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let start = self.pos;
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
                    Ok(Some((str, char, self.add_to_map((start, self.pos)))))
                };
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        if to_end && !str.is_empty() {
            let char = str
                .chars()
                .last()
                .ok_or(E::Other("Fail to get last char".to_string()))?;
            self.pos += pos - if cursor_after_stop_char { 0 } else { 1 };
            Ok(Some((str, char, self.add_to_map((start, self.pos)))))
        } else {
            Ok(None)
        }
    }

    pub fn read_until_wt(
        &mut self,
        cursor_after_stop_char: bool,
    ) -> Result<Option<(String, char, usize)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let start = self.pos;
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
                    Ok(Some((str, char, self.add_to_map((start, self.pos)))))
                };
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        Ok(None)
    }
    pub fn read_until_close(
        &mut self,
        open: char,
        close: char,
        cursor_after_stop_char: bool,
    ) -> Result<Option<(String, usize)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let start = self.pos;
        let content = &self.content[self.pos..];
        let mut root_opened = false;
        let mut opened: i32 = 0;
        for char in content.chars() {
            pos += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            let writing = root_opened;
            if !serialized {
                if !root_opened && char != open && !char.is_whitespace() {
                    return Ok(None);
                } else if char == open {
                    root_opened = true;
                }
                if char == open {
                    opened += 1;
                }
                if char == close {
                    opened -= 1;
                }
                if char == close && opened == 0 {
                    return if str.is_empty() {
                        Ok(None)
                    } else {
                        self.pos += pos - if cursor_after_stop_char { 0 } else { 1 };
                        Ok(Some((str, self.add_to_map((start, self.pos)))))
                    };
                }
            }
            serialized = char == chars::SERIALIZING;
            if writing {
                str.push(char);
            }
        }
        Ok(None)
    }
    pub fn read_until_word(
        &mut self,
        targets: &[&str],
        cancel_on: &[char],
        cursor_after_stop: bool,
    ) -> Result<Option<(String, String, usize)>, E> {
        let mut pos: usize = 0;
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let mut unserialized: String = String::new();
        let start = self.pos;
        let content = &self.content[self.pos..];
        for char in content.chars() {
            pos += 1;
            if !char.is_ascii() {
                Err(E::NotAscii(char))?;
            }
            if !serialized && char != chars::SERIALIZING {
                unserialized.push(char);
            }
            if !serialized && cancel_on.contains(&char) {
                return Ok(None);
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
            for word in targets.iter() {
                if unserialized.ends_with(word) {
                    return if cursor_after_stop {
                        self.pos += pos;
                        Ok(Some((
                            str,
                            word.to_string(),
                            self.mapper.add((start, self.pos)),
                        )))
                    } else {
                        self.pos += pos - word.len();
                        Ok(Some((
                            str[0..pos - word.len()].to_string(),
                            word.to_string(),
                            self.mapper.add((start, self.pos)),
                        )))
                    };
                }
            }
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
    fn add_to_map(&mut self, pos: (usize, usize)) -> usize {
        self.mapper.add((self.offset + pos.0, self.offset + pos.1))
    }
}

pub fn read_file(filename: &PathBuf) -> Result<Vec<Component>, E> {
    if !filename.exists() {
        Err(E::FileNotExists(filename.to_string_lossy().to_string()))?
    }
    let mut mapper = Mapper::new();
    let mut reader = Reader::new(fs::read_to_string(filename)?, &mut mapper, 0);
    let mut imports: Vec<Import> = vec![];
    let context = Context {
        cwd: filename.parent().ok_or(E::NoFileParent)?.to_path_buf(),
    };
    while let Some(func) = Function::read(&mut reader)? {
        if let Some(fn_impl) = <Import as Implementation<Import, String>>::from(func, &context)? {
            imports.push(fn_impl);
        } else {
            Err(E::NotAllowedFunction)?
        }
    }
    let mut components: Vec<Component> = vec![];
    for import in imports.iter_mut() {
        components.append(&mut read_file(&import.path)?);
    }
    while let Some(component) = Component::read(&mut reader)? {
        components.push(component);
    }
    Ok(components)
}

#[cfg(test)]
mod test_reader {
    use crate::{error::E, reader::read_file};

    #[test]
    fn reading() -> Result<(), E> {
        let target = std::env::current_dir()
            .unwrap()
            .join("./src/reader/entry/tests/full/build.sibs");
        let components = read_file(&target)?;
        println!("{components:?}");
        assert!(!components.is_empty());
        Ok(())
    }
}
