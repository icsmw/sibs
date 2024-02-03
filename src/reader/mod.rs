pub mod chars;
pub mod entry;
pub mod error;
#[cfg(test)]
pub mod tests;
pub mod words;

use crate::{
    executors::{import::Import, Executor},
    inf::context::Context,
};
use entry::{Component, Function, Reading};
use error::E;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    {cell::RefCell, rc::Rc},
};

#[derive(Debug)]
pub struct Map {
    //          <id,    (from,  len  )>
    map: HashMap<usize, (usize, usize)>,
    content: String,
    index: usize,
}

impl Map {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            content: String::new(),
            index: 0,
        }
    }
    pub fn last(&self) -> Option<(usize, (usize, usize))> {
        if self.index > 0 {
            let index = self.index - 1;
            self.map.get(&index).map(|coors| (index, *coors))
        } else {
            None
        }
    }
    fn add(&mut self, from: usize, len: usize) -> usize {
        self.map.insert(self.index, (from, len));
        self.index += 1;
        self.index
    }
}

#[derive(Debug)]
pub struct MoveTo<'a> {
    bound: &'a mut Reader,
}
impl<'a> MoveTo<'a> {
    pub fn new(bound: &'a mut Reader) -> Self {
        Self { bound }
    }
    pub fn char(&mut self, chars: &[&char]) -> Option<char> {
        if self.bound.done() {
            return None;
        }
        let content = &self.bound.content[self.bound.pos..];
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                continue;
            }
            return if chars.contains(&&char) {
                self.bound.index(self.bound.pos, pos);
                self.bound.pos += pos + 1;
                Some(char)
            } else {
                None
            };
        }
        None
    }
    pub fn word(&mut self, words: &[&str]) -> Option<String> {
        if self.bound.done() {
            return None;
        }
        let content = &self.bound.content[self.bound.pos..];
        let mut matched: Option<(String, usize, usize)> = None;
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                continue;
            }
            let current = self.bound.pos + pos;
            let mut skipped = false;
            for word in words.iter() {
                if matched.is_some() {
                    break;
                }
                let next = current + (word.len() - 1);
                if next > self.bound.content.len() - 1 {
                    skipped = true;
                    continue;
                }
                let fragment = self.bound.content[current..=next].to_string();
                if fragment == *word {
                    matched = Some((fragment, next + 1, pos));
                }
            }
            if matched.is_some() || !skipped {
                break;
            }
        }
        if let Some((word, next, pos)) = matched {
            self.bound.index(self.bound.pos, pos);
            self.bound.pos = next;
            Some(word)
        } else {
            None
        }
    }
    pub fn whitespace(&mut self) -> bool {
        if self.bound.done() {
            return false;
        }
        let content = &self.bound.content[self.bound.pos..];
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                self.bound.index(self.bound.pos, pos);
                self.bound.pos += pos + 1;
                return true;
            }
        }
        false
    }
    pub fn next(&mut self) -> bool {
        if self.bound.pos < self.bound.content.len() {
            self.bound.pos += 1;
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn if_next(&mut self, target: &str) -> bool {
        let next = self.bound.pos + target.len();
        if next > self.bound.content.len() - 1 {
            return false;
        }
        let fragment = &self.bound.content[self.bound.pos..next];
        if fragment != target {
            return false;
        }
        self.bound.pos = next;
        true
    }
    pub fn end(&mut self) -> String {
        let rest = self.bound.rest().to_string();
        let pos = if !rest.is_empty() {
            self.bound.content.len()
        } else {
            self.bound.pos
        };
        self.bound.index(self.bound.pos, pos - self.bound.pos);
        self.bound.pos = pos;
        rest
    }
}

#[derive(Debug)]
pub struct Until<'a> {
    bound: &'a mut Reader,
}
impl<'a> Until<'a> {
    pub fn new(bound: &'a mut Reader) -> Self {
        Self { bound }
    }
    pub fn char(&mut self, targets: &[&char]) -> Option<(String, char)> {
        if self.bound.done() {
            return None;
        }
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let whitespace = targets.iter().any(|c| **c == chars::WS);
        let content = &self.bound.content[self.bound.pos..];
        for (pos, char) in content.chars().enumerate() {
            if !serialized && (targets.contains(&&char) || (char.is_whitespace() && whitespace)) {
                self.bound.index(self.bound.pos, pos);
                self.bound.pos += pos;
                return Some((str, char));
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        None
    }
    pub fn word(&mut self, targets: &[&str]) -> Option<(String, String)> {
        if self.bound.done() {
            return None;
        }
        let cancel_on = self.bound.chars;
        self.bound.chars = &[];
        let mut serialized: bool = false;
        let mut clean: String = String::new();
        let content = &self.bound.content[self.bound.pos..];
        for (pos, char) in content.chars().enumerate() {
            if !serialized && char != chars::SERIALIZING {
                clean.push(char);
            }
            if !serialized && cancel_on.contains(&&char) {
                return None;
            }
            serialized = char == chars::SERIALIZING;
            for word in targets.iter() {
                if clean.ends_with(word) {
                    let next_pos = self.bound.pos + pos - (word.len() - 1);
                    let read = self.bound.content[self.bound.pos..next_pos].to_string();
                    self.bound.index(self.bound.pos, pos - (word.len() - 1));
                    self.bound.pos = next_pos;
                    return Some((read, word.to_string()));
                }
            }
        }
        None
    }
    pub fn whitespace(&mut self) -> Option<String> {
        if self.bound.done() {
            return None;
        }
        let content = &self.bound.content[self.bound.pos..];
        let mut serialized: bool = false;
        let mut str: String = String::new();
        for (pos, char) in content.chars().enumerate() {
            if !serialized && char != chars::SERIALIZING && char.is_whitespace() {
                self.bound.index(self.bound.pos, pos);
                self.bound.pos += pos;
                return Some(str);
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        None
    }
}

#[derive(Debug)]
pub struct Contains<'a> {
    bound: &'a mut Reader,
}
impl<'a> Contains<'a> {
    pub fn new(bound: &'a mut Reader) -> Self {
        Self { bound }
    }
    pub fn char(&mut self, target: &char) -> bool {
        if self.bound.done() {
            return false;
        }
        let content = &self.bound.content[self.bound.pos..];
        let mut serialized: bool = false;
        for char in content.chars() {
            if serialized || char == chars::SERIALIZING {
                serialized = char == chars::SERIALIZING;
                continue;
            }
            if char == *target {
                return true;
            }
        }
        false
    }
    pub fn word(&mut self, targets: &[&str]) -> bool {
        if self.bound.done() {
            return false;
        }
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.bound.content[self.bound.pos..];
        for char in content.chars() {
            if !serialized && char != chars::SERIALIZING {
                str.push(char);
            }
            serialized = char == chars::SERIALIZING;
            for word in targets.iter() {
                if str.ends_with(word) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct Group<'a> {
    bound: &'a mut Reader,
}
impl<'a> Group<'a> {
    pub fn new(bound: &'a mut Reader) -> Self {
        Self { bound }
    }
    pub fn between(&mut self, open: &char, close: &char) -> Option<String> {
        if self.bound.done() {
            return None;
        }
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.bound.content[self.bound.pos..];
        let mut opened: Option<usize> = None;
        let mut count: i32 = 0;
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() && opened.is_none() {
                continue;
            }
            if !char.is_whitespace() && opened.is_none() && char != *open {
                return None;
            }
            if char == *open && !serialized {
                if opened.is_none() {
                    opened = Some(self.bound.pos + pos + 1);
                    count += 1;
                    continue;
                }
                count += 1;
            } else if char == *close && !serialized {
                count -= 1;
                if let (0, Some(opened)) = (count, opened) {
                    self.bound.index(opened, str.len());
                    self.bound.pos += pos + 1;
                    return Some(str);
                }
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        None
    }
    pub fn closed(&mut self, border: &char) -> Option<String> {
        if self.bound.done() {
            return None;
        }
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.bound.content[self.bound.pos..];
        let mut opened: Option<usize> = None;
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() && opened.is_none() {
                continue;
            }
            if !char.is_whitespace() && opened.is_none() && char != *border {
                return None;
            }
            if char == *border && !serialized {
                if let Some(opened) = opened {
                    self.bound.index(opened, str.len());
                    self.bound.pos += pos + 1;
                    return Some(str);
                } else {
                    opened = Some(self.bound.pos + pos + 1);
                    continue;
                }
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        None
    }
}
#[derive(Debug)]
pub struct State<'a> {
    bound: &'a mut Reader,
}
impl<'a> State<'a> {
    pub fn new(bound: &'a mut Reader) -> Self {
        Self { bound }
    }
    pub fn set(&mut self) {
        self.bound.fixed = Some(self.bound.pos);
    }
    pub fn restore(&mut self) -> Result<(), E> {
        self.bound.pos = self.bound.fixed.ok_or(E::EmptyGroup)?;
        Ok(())
    }
}
#[derive(Debug)]
pub struct SeekTo<'a> {
    bound: &'a mut Reader,
}
impl<'a> SeekTo<'a> {
    pub fn new(bound: &'a mut Reader) -> Self {
        Self { bound }
    }
    pub fn char(&mut self, target: &char) -> bool {
        if self.bound.done() {
            return false;
        }
        let content = &self.bound.content[self.bound.pos..];
        let mut serialized: bool = false;
        for (pos, char) in content.chars().enumerate() {
            if serialized || char == chars::SERIALIZING {
                serialized = char == chars::SERIALIZING;
                continue;
            }
            if char == *target {
                self.bound.pos += pos;
                return true;
            }
        }
        false
    }
}
#[derive(Debug)]
pub struct Next<'a> {
    bound: &'a Reader,
}
impl<'a> Next<'a> {
    pub fn new(bound: &'a Reader) -> Self {
        Self { bound }
    }
    pub fn char(&self) -> Option<char> {
        if self.bound.done() {
            return None;
        }
        self.bound.content[self.bound.pos..].chars().next()
    }
}

#[derive(Debug)]
pub struct Prev<'a> {
    bound: &'a Reader,
}
impl<'a> Prev<'a> {
    pub fn new(bound: &'a Reader) -> Self {
        Self { bound }
    }
    pub fn nth(&self, offset: usize) -> Option<char> {
        if self.bound.pos < offset {
            return None;
        }
        self.bound.content.chars().nth(self.bound.pos - offset)
    }
    pub fn word(&self, len: usize) -> Option<String> {
        if self.bound.pos < len {
            return None;
        }
        Some(self.bound.content[(self.bound.pos - len)..self.bound.pos].to_string())
    }
}
#[derive(Debug)]
pub struct Token {
    pub content: String,
    pub id: usize,
    pub from: usize,
    pub len: usize,
    pub bound: Reader,
}

#[derive(Debug)]
pub struct Reader {
    content: String,
    pos: usize,
    chars: &'static [&'static char],
    fixed: Option<usize>,
    _map: Rc<RefCell<Map>>,
    _offset: usize,
    _recent: usize,
}

impl Reader {
    pub fn new(content: String) -> Self {
        let mut map = Map::new();
        map.content = content.clone();
        Self {
            content,
            pos: 0,
            chars: &[],
            fixed: None,
            _offset: 0,
            _map: Rc::new(RefCell::new(map)),
            _recent: 0,
        }
    }
    pub fn inherit(content: String, map: Rc<RefCell<Map>>, offset: usize) -> Self {
        Self {
            content,
            pos: 0,
            chars: &[],
            fixed: None,
            _offset: offset,
            _map: map,
            _recent: 0,
        }
    }
    pub fn move_to(&mut self) -> MoveTo<'_> {
        MoveTo::new(self)
    }
    pub fn seek_to(&mut self) -> SeekTo<'_> {
        SeekTo::new(self)
    }
    pub fn until(&mut self) -> Until<'_> {
        Until::new(self)
    }
    pub fn contains(&mut self) -> Contains<'_> {
        Contains::new(self)
    }
    pub fn group(&mut self) -> Group<'_> {
        Group::new(self)
    }
    pub fn state(&mut self) -> State<'_> {
        State::new(self)
    }
    pub fn next(&self) -> Next<'_> {
        Next::new(self)
    }
    pub fn prev(&self) -> Prev<'_> {
        Prev::new(self)
    }
    pub fn cancel_on(&mut self, chars: &'static [&'static char]) -> &mut Self {
        self.chars = chars;
        self
    }
    pub fn rest(&self) -> &str {
        &self.content[self.pos..]
    }
    pub fn trim(&mut self) {
        let content = &self.content[self.pos..];
        for (pos, char) in content.chars().enumerate() {
            if !char.is_whitespace() {
                self.pos += pos;
                return;
            }
        }
    }
    pub fn index(&mut self, from: usize, len: usize) {
        self._map.borrow_mut().add(from + self._offset, len);
    }
    pub fn token(&self) -> Result<Token, E> {
        let content = self._map.borrow().content.to_string();
        self._map
            .borrow_mut()
            .last()
            .map(|(id, (from, len))| {
                let value = if len == 0 {
                    String::new()
                } else {
                    content[from..=(from + len - 1)].to_string()
                };
                Token {
                    content: value.to_string(),
                    id,
                    from,
                    len,
                    bound: Reader::inherit(value, self._map.clone(), from),
                }
            })
            .ok_or(E::FailGetToken)
    }
    pub fn done(&self) -> bool {
        self.pos == self.content.len()
    }
    pub fn unserialize(content: &str) -> String {
        content
            .to_string()
            .replace("\\\"", "\"")
            .replace("\\ ", " ")
    }
    pub fn serialize(content: &str) -> String {
        content
            .to_string()
            .replace('\"', "\\\"")
            .replace(' ', "\\ ")
    }
    pub fn is_ascii_alphabetic_and_alphanumeric(content: &str, exceptions: &[&char]) -> bool {
        for char in content.chars() {
            if !char.is_ascii_alphanumeric()
                && !char.is_ascii_alphabetic()
                && !exceptions.contains(&&char)
            {
                return false;
            }
        }
        true
    }
    #[cfg(test)]
    pub fn recent(&mut self) -> &str {
        if self.pos == 0 {
            ""
        } else {
            let readed = &self.content[self._recent..self.pos];
            self._recent = self.pos;
            readed
        }
    }
}

#[cfg(test)]
mod test_walker {
    use crate::reader::Reader;

    #[test]
    fn until_whitespace() {
        let words = ["one", "@two", "$%^_0", r"\ a\ b"];
        let splitters = [" ", "\t", " \t "];
        let mut count = 0;
        splitters.iter().for_each(|splitter| {
            let mut bound = Reader::new(words.join(splitter));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let read = if let Some(read) = bound.until().whitespace() {
                    read
                } else {
                    bound.move_to().end()
                };
                let token = bound.token().unwrap();
                assert_eq!(read, *word);
                assert_eq!(token.content, *word);
                assert_eq!(token.from, cursor);
                assert_eq!(token.len, word.len());
                cursor += word.len() + splitter.len();
                bound.trim();
                count += 1;
            });
        });
        assert_eq!(count, words.len() * splitters.len());
    }
    #[test]
    fn until_char() {
        let words = ["one", "two", r"\$%^\_0", r"a\@b"];
        let targets = ['@', '$', '_'];
        let mut count = 0;
        targets.iter().for_each(|target| {
            let mut bound = Reader::new(words.join(&target.to_string()));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let (read, char) = if let Some((read, char)) = bound.until().char(&[target]) {
                    assert!(bound.move_to().next());
                    (read, char)
                } else {
                    (bound.move_to().end(), *target)
                };
                let token = bound.token().unwrap();
                assert_eq!(read, *word);
                assert_eq!(char, *target);
                assert_eq!(token.content, *word);
                assert_eq!(token.from, cursor);
                assert_eq!(token.len, word.len());
                cursor += word.len() + 1;
                count += 1;
            });
        });
        assert_eq!(count, words.len() * targets.len());
    }
    #[test]
    fn until_word() {
        let words = ["one", "two", r"\$\>\!%^\=_0", r"a\>b"];
        let targets = [">", "==", "!=", "=>"];
        let mut count = 0;
        targets.iter().for_each(|target| {
            let mut bound = Reader::new(words.join(target.as_ref()));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let (read, stopped) = if let Some((read, stopped)) = bound.until().word(&[*target])
                {
                    assert!(bound.move_to().if_next(&stopped));
                    (read, stopped)
                } else {
                    (bound.move_to().end(), target.to_string())
                };
                let token = bound.token().unwrap();
                assert_eq!(read, *word);
                assert_eq!(stopped, *target);
                assert_eq!(token.content, *word);
                assert_eq!(token.from, cursor);
                assert_eq!(token.len, word.len());
                cursor += word.len() + target.len();
                count += 1;
            });
        });
        assert_eq!(count, words.len() * targets.len());
    }
    #[test]
    fn move_to_char() {
        let words = ["    ", "\t\t\t\n\n\n", "\t \n \t \n"];
        let targets = ['@', '$', '_'];
        let mut count = 0;
        let times = 4;
        words.iter().for_each(|word| {
            targets.iter().for_each(|target| {
                let mut content = String::new();
                for _ in 0..times {
                    content = format!("{content}{word}{target}");
                }
                let mut bound = Reader::new(content);
                for n in 0..times {
                    let stopped = bound.move_to().char(&[target]).unwrap();
                    let token = bound.token().unwrap();
                    assert_eq!(stopped, *target);
                    assert_eq!(token.content, *word);
                    let from = n * (word.len() + 1);
                    assert_eq!(token.from, from);
                    assert_eq!(token.len, word.len());
                    count += 1;
                }
            });
        });
        assert_eq!(count, words.len() * targets.len() * times);
    }
    #[test]
    fn move_to_word() {
        let words = ["    ", "\t\t\t\n\n\n", "\t \n \t \n"];
        let targets = [">", "==", "!=", "=>"];
        let mut count = 0;
        let times = 4;
        words.iter().for_each(|word| {
            targets.iter().for_each(|target| {
                let mut content = String::new();
                for _ in 0..times {
                    content = format!("{content}{word}{target}");
                }
                let mut bound = Reader::new(content);
                for n in 0..times {
                    let stopped = bound.move_to().word(&[target]).unwrap();
                    let token = bound.token().unwrap();
                    assert_eq!(stopped, *target);
                    assert_eq!(token.content, *word);
                    let from = n * (word.len() + target.len());
                    assert_eq!(token.from, from);
                    assert_eq!(token.len, word.len());
                    count += 1;
                }
            });
        });
        assert_eq!(count, words.len() * targets.len() * times);
    }
    #[test]
    fn move_to_whitespace() {
        let word = "__________";
        let whitespaces = [' ', '\t', '\n'];
        let mut count = 0;
        let times = 4;
        whitespaces.iter().for_each(|whitespace| {
            let mut content = String::new();
            for _ in 0..times {
                content = format!("{content}{word}{whitespace}");
            }
            let mut bound = Reader::new(content);
            for n in 0..times {
                assert!(bound.move_to().whitespace());
                let token = bound.token().unwrap();
                assert_eq!(token.content, *word);
                let from = n * (word.len() + 1);
                assert_eq!(token.from, from);
                assert_eq!(token.len, word.len());
                count += 1;
            }
        });
        assert_eq!(count, whitespaces.len() * times);
    }
    #[test]
    fn contains_char() {
        let word = "_________";
        let chars = ['@', '$', '%'];
        let mut count = 0;
        chars.iter().for_each(|char| {
            let mut bound = Reader::new(format!("{char}{word}"));
            assert!(bound.contains().char(char));
            let mut bound = Reader::new(format!(r"\\{char}{char}{word}"));
            assert!(bound.contains().char(char));
            let mut bound = Reader::new(format!(r"\\{char}{word}"));
            assert!(!bound.contains().char(char));
            count += 1;
        });
        assert_eq!(count, chars.len());
    }
    #[test]
    fn contains_word() {
        let word = "_________";
        let targets = [">", "==", "!=", "=>"];
        let mut count = 0;
        targets.iter().for_each(|target| {
            let mut bound = Reader::new(format!("{target}{word}"));
            assert!(bound.contains().word(&[target]));
            let mut bound = Reader::new(format!(r"\\{target}{target}{word}"));
            assert!(bound.contains().word(&[target]));
            let mut bound = Reader::new(format!(r"\\{target}{word}"));
            assert!(!bound.contains().word(&[target]));
            count += 1;
        });
        assert_eq!(count, targets.len());
    }
    #[test]
    fn group_between() {
        let noise = "abcdefg123456";
        let borders = [('{', '}'), ('<', '>'), ('[', ']'), ('>', '<')];
        let mut count = 0;
        borders.iter().for_each(|(left, right)| {
            {
                // Nested groups
                let content = format!("{left}{noise}{right}{noise}\\{left}{noise}\\{right}{noise}");
                let mut bound = Reader::new(format!(" \t\n {left}{content}{right}{noise}"));
                let between = bound.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let mut bound = Reader::new(between);
                let between = bound.group().between(left, right).unwrap();
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, noise);
            }
            {
                // Nested shifted groups
                let content = format!("{noise}\\{left}{left}{noise}{right}\\{right}{noise}");
                let mut bound = Reader::new(format!("{left}{content}{right}{noise}"));
                let between = bound.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let mut bound = Reader::new(between);
                bound.until().char(&[left]);
                let between = bound.group().between(left, right).unwrap();
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, noise);
            }
            {
                // Following groups with spaces between
                let content = format!("{noise}\\{left}{noise}\\{right}{noise}");
                let mut bound = Reader::new(format!(
                    "{left}{content}{right} \t \n{left}{content}{right}"
                ));
                let between = bound.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let between = bound.group().between(left, right).unwrap();
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, content);
            }
            {
                // Following groups without spaces
                let content = format!("{noise}\\{left}{noise}\\{right}{noise}");
                let mut bound =
                    Reader::new(format!("{left}{content}{right}{left}{content}{right}"));
                let between = bound.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let between = bound.group().between(left, right).unwrap();
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, content);
            }
            count += 1;
        });
        assert_eq!(count, borders.len());
    }
    #[test]
    fn group_closed() {
        let noise = "abcdefg123456";
        let borders = ['"', '|', '\''];
        let mut count = 0;
        borders.iter().for_each(|border| {
            {
                let mut bound = Reader::new(format!(" \t\n {border}{noise}{border}"));
                let between = bound.group().closed(border).unwrap();
                assert_eq!(between, noise);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
            }
            {
                let content = format!("\\{border}{noise}\\{border}");
                let mut bound = Reader::new(format!("{border}{content}{border}"));
                let between = bound.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
            }
            {
                // Following groups without spaces
                let content = format!("\\{border}{noise}\\{border}");
                let mut bound = Reader::new(format!(
                    "{border}{content}{border}{border}{content}{border}"
                ));
                let between = bound.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let between = bound.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
            }
            {
                // Following groups with spaces
                let content = format!("\\{border}{noise}\\{border}");
                let mut bound = Reader::new(format!(
                    "{border}{content}{border} \n \t{border}{content}{border}"
                ));
                let between = bound.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let between = bound.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
            }
            count += 1;
        });
        assert_eq!(count, borders.len());
    }
    #[test]
    fn mapping() {
        let noise = "=================";
        let inner = format!("<{noise}>{noise}");
        let mut bound = Reader::new(format!("[{inner}]"));
        let between = bound.group().between(&'[', &']').unwrap();
        assert_eq!(between, inner);
        let mut token = bound.token().unwrap();
        assert_eq!(token.content, inner);
        assert_eq!(token.from, 1);
        assert_eq!(token.len, inner.len());
        let between = token.bound.group().between(&'<', &'>').unwrap();
        assert_eq!(between, noise);
        let nested_token = token.bound.token().unwrap();
        assert_eq!(nested_token.content, noise);
        assert_eq!(nested_token.from, 2);
        assert_eq!(nested_token.len, noise.len());
    }
    #[test]
    fn to_end() {
        let noise = "=================";
        let mut bound = Reader::new(noise.to_string());
        let full = bound.move_to().end();
        assert_eq!(full, noise);
        let token = bound.token().unwrap();
        assert_eq!(token.content, noise);
        assert_eq!(token.from, 0);
        assert_eq!(token.len, noise.len());
        let mut bound = Reader::new(format!("{noise}@{noise}"));
        bound.until().char(&[&'@']).unwrap();
        bound.move_to().next();
        let rest = bound.move_to().end();
        assert_eq!(rest, noise);
        let token = bound.token().unwrap();
        assert_eq!(token.content, noise);
        assert_eq!(token.from, noise.len() + 1);
        assert_eq!(token.len, noise.len());
    }
    #[test]
    fn seek_to() {
        let noise = "=================";
        let mut bound = Reader::new(format!("{noise}@{noise}@{noise}"));
        bound.seek_to().char(&'@');
        assert_eq!(bound.pos, noise.len());
        bound.seek_to().char(&'@');
        assert_eq!(bound.pos, noise.len());
        bound.move_to().next();
        bound.seek_to().char(&'@');
        assert_eq!(bound.pos, noise.len() * 2 + 1);
    }
}

use std::{future::Future, pin::Pin};

pub fn read_file<'a>(
    cx: &'a mut Context,
) -> Pin<Box<dyn Future<Output = Result<Vec<Component>, E>> + 'a>> {
    Box::pin(async {
        if !cx.scenario.filename.exists() {
            Err(E::FileNotExists(
                cx.scenario.filename.to_string_lossy().to_string(),
            ))?
        }
        let mut reader = Reader::new(fs::read_to_string(&cx.scenario.filename)?);
        let mut imports: Vec<PathBuf> = vec![];
        while let Some(mut func) = Function::read(&mut reader)? {
            let handle = cx
                .get_fn(&Import::get_name())
                .ok_or(E::FunctionIsNotRegistred)?;
            imports.push(
                handle(&mut func, cx)
                    .await?
                    .ok_or(E::NotAllowedFunction)?
                    .get_as::<PathBuf>()
                    .ok_or(E::InvalidFunctionReturn)?
                    .clone(),
            );
        }
        let mut components: Vec<Component> = vec![];
        for import_path in imports.iter_mut() {
            let mut cx = Context::from_filename(import_path)?;
            components.append(&mut read_file(&mut cx).await?);
        }
        while let Some(component) = Component::read(&mut reader)? {
            components.push(component);
        }
        Ok(components)
    })
}

#[cfg(test)]
mod test_reader {
    use crate::{
        inf::context::Context,
        reader::{error::E, read_file},
    };

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let target = std::env::current_dir()
            .unwrap()
            .join("./src/tests/normal/full/build.sibs");
        let mut cx = Context::from_filename(&target)?;
        let components = read_file(&mut cx).await?;
        assert_eq!(components.len(), 9);
        Ok(())
    }
}
