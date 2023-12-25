use crate::{
    error::{LocatedError, E},
    reader::chars,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    map: HashMap<usize, (usize, usize)>,
    index: usize,
}

impl Map {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
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
    fn add(&mut self, pos: (usize, usize)) -> usize {
        self.map.insert(self.index, pos);
        self.index += 1;
        self.index
    }
}

#[derive(Debug)]
pub struct MoveTo<'a> {
    walker: &'a mut Walker,
}
impl<'a> MoveTo<'a> {
    pub fn new(walker: &'a mut Walker) -> Self {
        Self { walker }
    }
    pub fn char(&mut self, chars: &[&char]) -> Option<char> {
        let content = &self.walker.content[self.walker.pos..];
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                continue;
            }
            return if chars.contains(&&char) {
                self.walker
                    .map
                    .add((self.walker.pos, self.walker.pos + pos - 1));
                self.walker.pos += pos + 1;
                Some(char)
            } else {
                None
            };
        }
        None
    }
    pub fn word(&mut self, words: &[&str]) -> Option<String> {
        let content = &self.walker.content[self.walker.pos..];
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                continue;
            }
            for word in words.iter() {
                let current = self.walker.pos + pos;
                let next = current + (word.len() - 1);
                if next > self.walker.content.len() - 1 {
                    continue;
                }
                let fragment = &self.walker.content[current..=next];
                if fragment == *word {
                    self.walker.map.add((self.walker.pos, current - 1));
                    self.walker.pos = next + 1;
                    return Some(fragment.to_string());
                }
            }
        }
        None
    }
    pub fn whitespace(&mut self) -> bool {
        let content = &self.walker.content[self.walker.pos..];
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                self.walker
                    .map
                    .add((self.walker.pos, self.walker.pos + pos - 1));
                self.walker.pos += pos + 1;
                return true;
            }
        }
        false
    }
    pub fn next(&mut self) -> bool {
        if self.walker.pos < self.walker.content.len() - 1 {
            self.walker.pos += 1;
            true
        } else {
            false
        }
    }
    pub fn prev(&mut self) -> bool {
        if self.walker.pos > 0 {
            self.walker.pos -= 1;
            true
        } else {
            false
        }
    }
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
}

#[derive(Debug)]
pub struct Until<'a> {
    walker: &'a mut Walker,
}

impl<'a> Until<'a> {
    pub fn new(walker: &'a mut Walker) -> Self {
        Self { walker }
    }
    pub fn char(&mut self, targets: &[char]) -> Option<(String, char)> {
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.walker.content[self.walker.pos..];
        for (pos, char) in content.chars().enumerate() {
            if !serialized && targets.contains(&char) {
                return if str.is_empty() {
                    None
                } else {
                    self.walker
                        .map
                        .add((self.walker.pos, self.walker.pos + pos - 1));
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
                    self.walker.map.add((self.walker.pos, next_pos - 1));
                    self.walker.pos = next_pos;
                    return Some((read, word.to_string()));
                }
            }
        }
        None
    }
    pub fn whitespace(&mut self) -> Option<String> {
        let content = &self.walker.content[self.walker.pos..];
        let mut pos: usize = 0;
        let mut serialized: bool = false;
        let mut str: String = String::new();
        for char in content.chars() {
            if serialized || char == chars::SERIALIZING {
                serialized = char == chars::SERIALIZING;
                pos += 1;
                continue;
            }
            if char.is_whitespace() {
                self.walker
                    .map
                    .add((self.walker.pos, self.walker.pos + pos - 1));
                self.walker.pos += pos;
                return Some(str);
            }
            str.push(char);
            pos += 1;
        }
        None
    }
}

#[derive(Debug)]
pub struct Has<'a> {
    walker: &'a mut Walker,
}

impl<'a> Has<'a> {
    pub fn new(walker: &'a mut Walker) -> Self {
        Self { walker }
    }
    pub fn char(&mut self, target: char) -> bool {
        let content = &self.walker.content[self.walker.pos..];
        let mut serialized: bool = false;
        for char in content.chars() {
            if serialized || char == chars::SERIALIZING {
                serialized = char == chars::SERIALIZING;
                continue;
            }
            if char == target {
                return true;
            }
        }
        false
    }
    pub fn word(&mut self, targets: &[&str]) -> bool {
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
pub struct Token {
    pub content: String,
    pub id: usize,
    pub coors: (usize, usize),
}

#[derive(Debug)]
pub struct Walker {
    content: String,
    pos: usize,
    chars: &'static [&'static char],
    map: Map,
}

impl Walker {
    pub fn new(content: String) -> Self {
        Self {
            content,
            pos: 0,
            chars: &[],
            map: Map::new(),
        }
    }
    pub fn move_to(&mut self) -> MoveTo<'_> {
        MoveTo::new(self)
    }
    pub fn until(&mut self) -> Until<'_> {
        Until::new(self)
    }
    pub fn has(&mut self) -> Has<'_> {
        Has::new(self)
    }
    pub fn cancel_on(&mut self, chars: &'static [&'static char]) -> &mut Self {
        self.chars = chars;
        self
    }
    pub fn between(&mut self, open: &char, close: &char) -> Option<String> {
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.content[self.pos..];
        let mut root_opened = false;
        let mut opened: i32 = 0;
        let mut opened_on_pos = self.pos;
        for (pos, char) in content.chars().enumerate() {
            let writing = root_opened;
            if !serialized {
                if !root_opened && char != *open && !char.is_whitespace() {
                    return None;
                } else if char == *open {
                    if !root_opened {
                        opened_on_pos = self.pos + pos + 1;
                    }
                    root_opened = true;
                }
                if char == *open {
                    opened += 1;
                }
                if char == *close {
                    opened -= 1;
                }
                if char == *close && opened == 0 {
                    return if str.is_empty() {
                        None
                    } else {
                        self.map.add((opened_on_pos, self.pos + pos - 1));
                        self.pos += pos + 1;
                        Some(str)
                    };
                }
            }
            serialized = char == chars::SERIALIZING;
            if writing {
                str.push(char);
            }
        }
        None
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
    pub fn to_end(&mut self) -> String {
        let rest = self.rest().to_string();
        let pos = if !rest.is_empty() {
            self.content.len() - 1
        } else {
            self.pos
        };
        self.map.add((self.pos, pos));
        self.pos = pos;
        rest
    }
    pub fn token(&self) -> Option<Token> {
        self.map.last().map(|(id, coors)| Token {
            content: self.content[coors.0..=coors.1].to_string(),
            id,
            coors,
        })
    }
}

#[cfg(test)]
mod test_walker {
    use crate::reader::walker::Walker;

    #[test]
    fn until_whitespace() {
        let words = ["one", "@two", "$%^_0", r"a\ b"];
        let splitters = [" ", "\t", " \t "];
        let mut count = 0;
        splitters.iter().for_each(|splitter| {
            let mut walker = Walker::new(words.join(splitter));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let read = if let Some(read) = walker.until().whitespace() {
                    read
                } else {
                    walker.to_end()
                };
                let token = walker.token().unwrap();
                assert_eq!(read, *word);
                assert_eq!(token.content, *word);
                assert_eq!(token.coors, (cursor, cursor + word.len() - 1));
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
            let mut walker = Walker::new(words.join(&target.to_string()));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let (read, char) = if let Some((read, char)) = walker.until().char(&[*target]) {
                    assert!(walker.move_to().next());
                    (read, char)
                } else {
                    (walker.to_end(), *target)
                };
                let token = walker.token().unwrap();
                assert_eq!(read, *word);
                assert_eq!(char, *target);
                assert_eq!(token.content, *word);
                assert_eq!(token.coors, (cursor, cursor + word.len() - 1));
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
            let mut walker = Walker::new(words.join(target.as_ref()));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let (read, stopped) = if let Some((read, stopped)) = walker.until().word(&[*target])
                {
                    assert!(walker.move_to().if_next(&stopped));
                    (read, stopped)
                } else {
                    (walker.to_end(), target.to_string())
                };
                let token = walker.token().unwrap();
                assert_eq!(read, *word);
                assert_eq!(stopped, *target);
                assert_eq!(token.content, *word);
                assert_eq!(token.coors, (cursor, cursor + word.len() - 1));
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
                let mut walker = Walker::new(content);
                for n in 0..times {
                    let stopped = walker.move_to().char(&[target]).unwrap();
                    let token = walker.token().unwrap();
                    assert_eq!(stopped, *target);
                    assert_eq!(token.content, *word);
                    let from = n * (word.len() + 1);
                    assert_eq!(token.coors, (from, from + word.len() - 1));
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
                let mut walker = Walker::new(content);
                for n in 0..times {
                    let stopped = walker.move_to().word(&[target]).unwrap();
                    let token = walker.token().unwrap();
                    assert_eq!(stopped, *target);
                    assert_eq!(token.content, *word);
                    let from = n * (word.len() + target.len());
                    assert_eq!(token.coors, (from, from + word.len() - 1));
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
            let mut walker = Walker::new(content);
            for n in 0..times {
                assert!(walker.move_to().whitespace());
                let token = walker.token().unwrap();
                assert_eq!(token.content, *word);
                let from = n * (word.len() + 1);
                assert_eq!(token.coors, (from, from + word.len() - 1));
                count += 1;
            }
        });
        assert_eq!(count, whitespaces.len() * times);
    }
    #[test]
    fn has_char() {
        let word = "_________";
        let chars = ['@', '$', '%'];
        let mut count = 0;
        chars.iter().for_each(|char| {
            let mut walker = Walker::new(format!("{char}{word}"));
            assert!(walker.has().char(*char));
            let mut walker = Walker::new(format!(r"\\{char}{char}{word}"));
            assert!(walker.has().char(*char));
            let mut walker = Walker::new(format!(r"\\{char}{word}"));
            assert!(!walker.has().char(*char));
            count += 1;
        });
        assert_eq!(count, chars.len());
    }
    #[test]
    fn has_word() {
        let word = "_________";
        let targets = [">", "==", "!=", "=>"];
        let mut count = 0;
        targets.iter().for_each(|target| {
            let mut walker = Walker::new(format!("{target}{word}"));
            assert!(walker.has().word(&[target]));
            let mut walker = Walker::new(format!(r"\\{target}{target}{word}"));
            assert!(walker.has().word(&[target]));
            let mut walker = Walker::new(format!(r"\\{target}{word}"));
            assert!(!walker.has().word(&[target]));
            count += 1;
        });
        assert_eq!(count, targets.len());
    }
    #[test]
    fn between() {
        let noise = "abcdefg123456";
        let borders = [('{', '}'), ('<', '>'), ('[', ']'), ('>', '<')];
        let mut count = 0;
        borders.iter().for_each(|(left, right)| {
            {
                // Nested groups
                let content = format!("{left}{noise}{right}{noise}\\{left}{noise}\\{right}{noise}");
                let mut walker = Walker::new(format!(" \t\n {left}{content}{right}{noise}"));
                let between = walker.between(left, right).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                let mut walker = Walker::new(between);
                let between = walker.between(left, right).unwrap();
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, noise);
            }
            {
                // Nested shifted groups
                let content = format!("{noise}\\{left}{left}{noise}{right}\\{right}{noise}");
                let mut walker = Walker::new(format!("{left}{content}{right}{noise}"));
                let between = walker.between(left, right).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                let mut walker = Walker::new(between);
                walker.until().char(&[*left]);
                let between = walker.between(left, right).unwrap();
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, noise);
            }
            {
                // Following groups with spaces between
                let content = format!("{noise}\\{left}{noise}\\{right}{noise}");
                let mut walker = Walker::new(format!(
                    "{left}{content}{right} \t \n{left}{content}{right}"
                ));
                let between = walker.between(left, right).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                let between = walker.between(left, right).unwrap();
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, content);
            }
            {
                // Following groups without spaces
                let content = format!("{noise}\\{left}{noise}\\{right}{noise}");
                let mut walker =
                    Walker::new(format!("{left}{content}{right}{left}{content}{right}"));
                let between = walker.between(left, right).unwrap();
                assert_eq!(between, content);
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                let between = walker.between(left, right).unwrap();
                let token = walker.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, content);
            }
            count += 1;
        });
        assert_eq!(count, borders.len());
    }
}
