pub mod chars;
pub mod entry;
pub mod words;

use crate::{
    context::Context,
    error::E,
    functions::{reader::import::Import, Implementation},
};
use entry::{Component, Function, Reading};
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
    walker: &'a mut Reader,
}
impl<'a> MoveTo<'a> {
    pub fn new(walker: &'a mut Reader) -> Self {
        Self { walker }
    }
    pub fn char(&mut self, chars: &[&char]) -> Option<char> {
        if self.walker.done() {
            return None;
        }
        let content = &self.walker.content[self.walker.pos..];
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                continue;
            }
            return if chars.contains(&&char) {
                self.walker.index(self.walker.pos, pos);
                self.walker.pos += pos + 1;
                Some(char)
            } else {
                None
            };
        }
        None
    }
    pub fn word(&mut self, words: &[&str]) -> Option<String> {
        if self.walker.done() {
            return None;
        }
        let content = &self.walker.content[self.walker.pos..];
        let mut matched: Option<(String, usize, usize)> = None;
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                continue;
            }
            let current = self.walker.pos + pos;
            let mut skipped = false;
            for word in words.iter() {
                if matched.is_some() {
                    break;
                }
                let next = current + (word.len() - 1);
                if next > self.walker.content.len() - 1 {
                    skipped = true;
                    continue;
                }
                let fragment = self.walker.content[current..=next].to_string();
                if fragment == *word {
                    matched = Some((fragment, next + 1, pos));
                }
            }
            if matched.is_some() || !skipped {
                break;
            }
        }
        if let Some((word, next, pos)) = matched {
            self.walker.index(self.walker.pos, pos);
            self.walker.pos = next;
            Some(word)
        } else {
            None
        }
    }
    pub fn whitespace(&mut self) -> bool {
        if self.walker.done() {
            return false;
        }
        let content = &self.walker.content[self.walker.pos..];
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                self.walker.index(self.walker.pos, pos);
                self.walker.pos += pos + 1;
                return true;
            }
        }
        false
    }
    pub fn next(&mut self) -> bool {
        if self.walker.pos < self.walker.content.len() {
            self.walker.pos += 1;
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn if_next(&mut self, target: &str) -> bool {
        let next = self.walker.pos + target.len();
        if next > self.walker.content.len() - 1 {
            return false;
        }
        let fragment = &self.walker.content[self.walker.pos..next];
        if fragment != target {
            return false;
        }
        self.walker.pos = next;
        true
    }
    pub fn end(&mut self) -> String {
        let rest = self.walker.rest().to_string();
        let pos = if !rest.is_empty() {
            self.walker.content.len()
        } else {
            self.walker.pos
        };
        self.walker.index(self.walker.pos, pos - self.walker.pos);
        self.walker.pos = pos;
        rest
    }
}

#[derive(Debug)]
pub struct Until<'a> {
    walker: &'a mut Reader,
}
impl<'a> Until<'a> {
    pub fn new(walker: &'a mut Reader) -> Self {
        Self { walker }
    }
    pub fn char(&mut self, targets: &[&char]) -> Option<(String, char)> {
        if self.walker.done() {
            return None;
        }
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let whitespace = targets.iter().any(|c| **c == chars::WS);
        let content = &self.walker.content[self.walker.pos..];
        for (pos, char) in content.chars().enumerate() {
            if !serialized && (targets.contains(&&char) || (char.is_whitespace() && whitespace)) {
                return if str.is_empty() {
                    None
                } else {
                    self.walker.index(self.walker.pos, pos);
                    self.walker.pos += pos;
                    Some((str, char))
                };
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        None
    }
    pub fn word(&mut self, targets: &[&str]) -> Option<(String, String)> {
        if self.walker.done() {
            return None;
        }
        let cancel_on = self.walker.chars;
        self.walker.chars = &[];
        let mut serialized: bool = false;
        let mut clean: String = String::new();
        let content = &self.walker.content[self.walker.pos..];
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
                    let next_pos = self.walker.pos + pos - (word.len() - 1);
                    let read = self.walker.content[self.walker.pos..next_pos].to_string();
                    self.walker.index(self.walker.pos, pos - (word.len() - 1));
                    self.walker.pos = next_pos;
                    return Some((read, word.to_string()));
                }
            }
        }
        None
    }
    pub fn whitespace(&mut self) -> Option<String> {
        if self.walker.done() {
            return None;
        }
        let content = &self.walker.content[self.walker.pos..];
        let mut serialized: bool = false;
        let mut str: String = String::new();
        for (pos, char) in content.chars().enumerate() {
            if !serialized && char != chars::SERIALIZING && char.is_whitespace() {
                self.walker.index(self.walker.pos, pos);
                self.walker.pos += pos;
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
    walker: &'a mut Reader,
}
impl<'a> Contains<'a> {
    pub fn new(walker: &'a mut Reader) -> Self {
        Self { walker }
    }
    pub fn char(&mut self, target: &char) -> bool {
        if self.walker.done() {
            return false;
        }
        let content = &self.walker.content[self.walker.pos..];
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
        if self.walker.done() {
            return false;
        }
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.walker.content[self.walker.pos..];
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
    walker: &'a mut Reader,
}
impl<'a> Group<'a> {
    pub fn new(walker: &'a mut Reader) -> Self {
        Self { walker }
    }
    pub fn between(&mut self, open: &char, close: &char) -> Option<String> {
        if self.walker.done() {
            return None;
        }
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.walker.content[self.walker.pos..];
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
                    opened = Some(self.walker.pos + pos + 1);
                    count += 1;
                    continue;
                }
                count += 1;
            } else if char == *close && !serialized {
                count -= 1;
                if let (0, Some(opened)) = (count, opened) {
                    self.walker.index(opened, str.len());
                    self.walker.pos += pos + 1;
                    return Some(str);
                }
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        None
    }
    pub fn closed(&mut self, border: &char) -> Option<String> {
        if self.walker.done() {
            return None;
        }
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.walker.content[self.walker.pos..];
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
                    self.walker.index(opened, str.len());
                    self.walker.pos += pos + 1;
                    return Some(str);
                } else {
                    opened = Some(self.walker.pos + pos + 1);
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
    walker: &'a mut Reader,
}
impl<'a> State<'a> {
    pub fn new(walker: &'a mut Reader) -> Self {
        Self { walker }
    }
    pub fn set(&mut self) {
        self.walker.fixed = Some(self.walker.pos);
    }
    pub fn restore(&mut self) -> Result<(), E> {
        self.walker.pos = self.walker.fixed.ok_or(E::EmptyGroup)?;
        Ok(())
    }
}
#[derive(Debug)]
pub struct SeekTo<'a> {
    walker: &'a mut Reader,
}
impl<'a> SeekTo<'a> {
    pub fn new(walker: &'a mut Reader) -> Self {
        Self { walker }
    }
    pub fn char(&mut self, target: &char) -> bool {
        if self.walker.done() {
            return false;
        }
        let content = &self.walker.content[self.walker.pos..];
        let mut serialized: bool = false;
        for (pos, char) in content.chars().enumerate() {
            if serialized || char == chars::SERIALIZING {
                serialized = char == chars::SERIALIZING;
                continue;
            }
            if char == *target {
                self.walker.pos += pos;
                return true;
            }
        }
        false
    }
}
#[derive(Debug)]
pub struct Next<'a> {
    walker: &'a mut Reader,
}
impl<'a> Next<'a> {
    pub fn new(walker: &'a mut Reader) -> Self {
        Self { walker }
    }
    pub fn char(&mut self) -> Option<char> {
        if self.walker.done() {
            return None;
        }
        self.walker.content[self.walker.pos..].chars().next()
    }
}
#[derive(Debug)]
pub struct Token {
    pub content: String,
    pub id: usize,
    pub from: usize,
    pub len: usize,
    pub walker: Reader,
}

#[derive(Debug)]
pub struct Reader {
    content: String,
    pos: usize,
    chars: &'static [&'static char],
    fixed: Option<usize>,
    _map: Rc<RefCell<Map>>,
    _offset: usize,
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
    pub fn next(&mut self) -> Next<'_> {
        Next::new(self)
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
                    walker: Reader::inherit(value, self._map.clone(), from),
                }
            })
            .ok_or(E::FailGetToken)
    }
    pub fn done(&self) -> bool {
        self.pos == self.content.len()
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

#[cfg(test)]
mod test_walker {
    use crate::reader::Reader;

    #[test]
    fn until_whitespace() {
        let words = ["one", "@two", "$%^_0", r"\ a\ b"];
        let splitters = [" ", "\t", " \t "];
        let mut count = 0;
        splitters.iter().for_each(|splitter| {
            let mut walker = Reader::new(words.join(splitter));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let read = if let Some(read) = walker.until().whitespace() {
                    read
                } else {
                    walker.move_to().end()
                };
                let token = walker.token().unwrap();
                assert_eq!(read, *word);
                assert_eq!(token.content, *word);
                assert_eq!(token.from, cursor);
                assert_eq!(token.len, word.len());
                cursor += word.len() + splitter.len();
                walker.trim();
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
            let mut walker = Reader::new(words.join(&target.to_string()));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let (read, char) = if let Some((read, char)) = walker.until().char(&[target]) {
                    assert!(walker.move_to().next());
                    (read, char)
                } else {
                    (walker.move_to().end(), *target)
                };
                let token = walker.token().unwrap();
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
            let mut walker = Reader::new(words.join(target.as_ref()));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let (read, stopped) = if let Some((read, stopped)) = walker.until().word(&[*target])
                {
                    assert!(walker.move_to().if_next(&stopped));
                    (read, stopped)
                } else {
                    (walker.move_to().end(), target.to_string())
                };
                let token = walker.token().unwrap();
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
                let mut walker = Reader::new(content);
                for n in 0..times {
                    let stopped = walker.move_to().char(&[target]).unwrap();
                    let token = walker.token().unwrap();
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
                let mut walker = Reader::new(content);
                for n in 0..times {
                    let stopped = walker.move_to().word(&[target]).unwrap();
                    let token = walker.token().unwrap();
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
            let mut walker = Reader::new(content);
            for n in 0..times {
                assert!(walker.move_to().whitespace());
                let token = walker.token().unwrap();
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
            let mut walker = Reader::new(format!("{char}{word}"));
            assert!(walker.contains().char(char));
            let mut walker = Reader::new(format!(r"\\{char}{char}{word}"));
            assert!(walker.contains().char(char));
            let mut walker = Reader::new(format!(r"\\{char}{word}"));
            assert!(!walker.contains().char(char));
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
            let mut walker = Reader::new(format!("{target}{word}"));
            assert!(walker.contains().word(&[target]));
            let mut walker = Reader::new(format!(r"\\{target}{target}{word}"));
            assert!(walker.contains().word(&[target]));
            let mut walker = Reader::new(format!(r"\\{target}{word}"));
            assert!(!walker.contains().word(&[target]));
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
                let mut walker = Reader::new(format!(" \t\n {left}{content}{right}{noise}"));
                let between = walker.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                let mut walker = Reader::new(between);
                let between = walker.group().between(left, right).unwrap();
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, noise);
            }
            {
                // Nested shifted groups
                let content = format!("{noise}\\{left}{left}{noise}{right}\\{right}{noise}");
                let mut walker = Reader::new(format!("{left}{content}{right}{noise}"));
                let between = walker.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                let mut walker = Reader::new(between);
                walker.until().char(&[left]);
                let between = walker.group().between(left, right).unwrap();
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, noise);
            }
            {
                // Following groups with spaces between
                let content = format!("{noise}\\{left}{noise}\\{right}{noise}");
                let mut walker = Reader::new(format!(
                    "{left}{content}{right} \t \n{left}{content}{right}"
                ));
                let between = walker.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                let between = walker.group().between(left, right).unwrap();
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, content);
            }
            {
                // Following groups without spaces
                let content = format!("{noise}\\{left}{noise}\\{right}{noise}");
                let mut walker =
                    Reader::new(format!("{left}{content}{right}{left}{content}{right}"));
                let between = walker.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                let between = walker.group().between(left, right).unwrap();
                let token = walker.token().unwrap();
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
                let mut walker = Reader::new(format!(" \t\n {border}{noise}{border}"));
                let between = walker.group().closed(border).unwrap();
                assert_eq!(between, noise);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
            }
            {
                let content = format!("\\{border}{noise}\\{border}");
                let mut walker = Reader::new(format!("{border}{content}{border}"));
                let between = walker.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
            }
            {
                // Following groups without spaces
                let content = format!("\\{border}{noise}\\{border}");
                let mut walker = Reader::new(format!(
                    "{border}{content}{border}{border}{content}{border}"
                ));
                let between = walker.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                let between = walker.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
            }
            {
                // Following groups with spaces
                let content = format!("\\{border}{noise}\\{border}");
                let mut walker = Reader::new(format!(
                    "{border}{content}{border} \n \t{border}{content}{border}"
                ));
                let between = walker.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                let between = walker.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
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
        let mut walker = Reader::new(format!("[{inner}]"));
        let between = walker.group().between(&'[', &']').unwrap();
        assert_eq!(between, inner);
        let mut token = walker.token().unwrap();
        assert_eq!(token.content, inner);
        assert_eq!(token.from, 1);
        assert_eq!(token.len, inner.len());
        let between = token.walker.group().between(&'<', &'>').unwrap();
        assert_eq!(between, noise);
        let nested_token = token.walker.token().unwrap();
        assert_eq!(nested_token.content, noise);
        assert_eq!(nested_token.from, 2);
        assert_eq!(nested_token.len, noise.len());
    }
    #[test]
    fn to_end() {
        let noise = "=================";
        let mut walker = Reader::new(noise.to_string());
        let full = walker.move_to().end();
        assert_eq!(full, noise);
        let token = walker.token().unwrap();
        assert_eq!(token.content, noise);
        assert_eq!(token.from, 0);
        assert_eq!(token.len, noise.len());
        let mut walker = Reader::new(format!("{noise}@{noise}"));
        walker.until().char(&[&'@']).unwrap();
        walker.move_to().next();
        let rest = walker.move_to().end();
        assert_eq!(rest, noise);
        let token = walker.token().unwrap();
        assert_eq!(token.content, noise);
        assert_eq!(token.from, noise.len() + 1);
        assert_eq!(token.len, noise.len());
    }
    #[test]
    fn seek_to() {
        let noise = "=================";
        let mut walker = Reader::new(format!("{noise}@{noise}@{noise}"));
        walker.seek_to().char(&'@');
        assert_eq!(walker.pos, noise.len());
        walker.seek_to().char(&'@');
        assert_eq!(walker.pos, noise.len());
        walker.move_to().next();
        walker.seek_to().char(&'@');
        assert_eq!(walker.pos, noise.len() * 2 + 1);
    }
}

pub fn read_file(filename: &PathBuf) -> Result<Vec<Component>, E> {
    if !filename.exists() {
        Err(E::FileNotExists(filename.to_string_lossy().to_string()))?
    }
    let mut reader = Reader::new(fs::read_to_string(filename)?);
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
