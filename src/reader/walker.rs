use crate::reader::chars;
use std::{
    collections::HashMap,
    {cell::RefCell, rc::Rc},
};

#[derive(Debug)]
pub struct Map {
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
    fn add(&mut self, pos: (usize, usize)) -> usize {
        self.map.insert(self.index, pos);
        self.index += 1;
        self.index
    }
}

#[derive(Debug)]
pub struct MoveTo<'a> {
    bound: &'a mut Walker,
}
impl<'a> MoveTo<'a> {
    pub fn new(bound: &'a mut Walker) -> Self {
        Self { bound }
    }
    pub fn char(&mut self, chars: &[&char]) -> Option<char> {
        let content = &self.bound.content[self.bound.pos..];
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                continue;
            }
            return if chars.contains(&&char) {
                self.bound.index((self.bound.pos, self.bound.pos + pos - 1));
                self.bound.pos += pos + 1;
                Some(char)
            } else {
                None
            };
        }
        None
    }
    pub fn word(&mut self, words: &[&str]) -> Option<String> {
        let content = &self.bound.content[self.bound.pos..];
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                continue;
            }
            for word in words.iter() {
                let current = self.bound.pos + pos;
                let next = current + (word.len() - 1);
                if next > self.bound.content.len() - 1 {
                    continue;
                }
                let fragment = self.bound.content[current..=next].to_string();
                if fragment == *word {
                    self.bound.index((self.bound.pos, current - 1));
                    self.bound.pos = next + 1;
                    return Some(fragment);
                }
            }
        }
        None
    }
    pub fn whitespace(&mut self) -> bool {
        let content = &self.bound.content[self.bound.pos..];
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() {
                self.bound.index((self.bound.pos, self.bound.pos + pos - 1));
                self.bound.pos += pos + 1;
                return true;
            }
        }
        false
    }
    pub fn next(&mut self) -> bool {
        if self.bound.pos < self.bound.content.len() - 1 {
            self.bound.pos += 1;
            true
        } else {
            false
        }
    }
    pub fn prev(&mut self) -> bool {
        if self.bound.pos > 0 {
            self.bound.pos -= 1;
            true
        } else {
            false
        }
    }
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
            self.bound.content.len() - 1
        } else {
            self.bound.pos
        };
        self.bound.index((self.bound.pos, pos));
        self.bound.pos = pos;
        rest
    }
}

#[derive(Debug)]
pub struct Until<'a> {
    bound: &'a mut Walker,
}
impl<'a> Until<'a> {
    pub fn new(bound: &'a mut Walker) -> Self {
        Self { bound }
    }
    pub fn char(&mut self, targets: &[char]) -> Option<(String, char)> {
        let mut str: String = String::new();
        let mut serialized: bool = false;
        let content = &self.bound.content[self.bound.pos..];
        for (pos, char) in content.chars().enumerate() {
            if !serialized && targets.contains(&char) {
                return if str.is_empty() {
                    None
                } else {
                    self.bound.index((self.bound.pos, self.bound.pos + pos - 1));
                    self.bound.pos += pos;
                    Some((str, char))
                };
            }
            serialized = char == chars::SERIALIZING;
            str.push(char);
        }
        None
    }
    pub fn word(&mut self, targets: &[&str]) -> Option<(String, String)> {
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
                    self.bound.index((self.bound.pos, next_pos - 1));
                    self.bound.pos = next_pos;
                    return Some((read, word.to_string()));
                }
            }
        }
        None
    }
    pub fn whitespace(&mut self) -> Option<String> {
        let content = &self.bound.content[self.bound.pos..];
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
                self.bound.index((self.bound.pos, self.bound.pos + pos - 1));
                self.bound.pos += pos;
                return Some(str);
            }
            str.push(char);
            pos += 1;
        }
        None
    }
}

#[derive(Debug)]
pub struct Contains<'a> {
    bound: &'a mut Walker,
}
impl<'a> Contains<'a> {
    pub fn new(bound: &'a mut Walker) -> Self {
        Self { bound }
    }
    pub fn char(&mut self, target: char) -> bool {
        let content = &self.bound.content[self.bound.pos..];
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
    bound: &'a mut Walker,
}
impl<'a> Group<'a> {
    pub fn new(bound: &'a mut Walker) -> Self {
        Self { bound }
    }
    pub fn between(&mut self, open: &char, close: &char) -> Option<String> {
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
                    self.bound.index((opened, self.bound.pos + pos - 1));
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
                    self.bound.index((opened, self.bound.pos + pos - 1));
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
pub struct Token {
    pub content: String,
    pub id: usize,
    pub coors: (usize, usize),
    pub bound: Walker,
}

#[derive(Debug)]
pub struct Walker {
    content: String,
    pos: usize,
    chars: &'static [&'static char],
    _map: Rc<RefCell<Map>>,
    _offset: usize,
}

impl Walker {
    pub fn new(content: String) -> Self {
        let mut map = Map::new();
        map.content = content.clone();
        Self {
            content,
            pos: 0,
            chars: &[],
            _offset: 0,
            _map: Rc::new(RefCell::new(map)),
        }
    }
    pub fn inherit(content: String, map: Rc<RefCell<Map>>, offset: usize) -> Self {
        Self {
            content,
            pos: 0,
            chars: &[],
            _offset: offset,
            _map: map,
        }
    }
    pub fn move_to(&mut self) -> MoveTo<'_> {
        MoveTo::new(self)
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
    pub fn index(&mut self, pos: (usize, usize)) {
        self._map
            .borrow_mut()
            .add((pos.0 + self._offset, pos.1 + self._offset));
    }
    pub fn token(&self) -> Option<Token> {
        let content = self._map.borrow().content.to_string();
        self._map.borrow_mut().last().map(|(id, coors)| {
            let value = content[coors.0..=coors.1].to_string();
            Token {
                content: value.to_string(),
                id,
                coors,
                bound: Walker::inherit(value, self._map.clone(), coors.0),
            }
        })
    }
}

#[cfg(test)]
mod test_walker {
    use crate::reader::bound::Walker;

    #[test]
    fn until_whitespace() {
        let words = ["one", "@two", "$%^_0", r"a\ b"];
        let splitters = [" ", "\t", " \t "];
        let mut count = 0;
        splitters.iter().for_each(|splitter| {
            let mut bound = Walker::new(words.join(splitter));
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
                assert_eq!(token.coors, (cursor, cursor + word.len() - 1));
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
            let mut bound = Walker::new(words.join(&target.to_string()));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let (read, char) = if let Some((read, char)) = bound.until().char(&[*target]) {
                    assert!(bound.move_to().next());
                    (read, char)
                } else {
                    (bound.move_to().end(), *target)
                };
                let token = bound.token().unwrap();
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
            let mut bound = Walker::new(words.join(target.as_ref()));
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
                let mut bound = Walker::new(content);
                for n in 0..times {
                    let stopped = bound.move_to().char(&[target]).unwrap();
                    let token = bound.token().unwrap();
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
                let mut bound = Walker::new(content);
                for n in 0..times {
                    let stopped = bound.move_to().word(&[target]).unwrap();
                    let token = bound.token().unwrap();
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
            let mut bound = Walker::new(content);
            for n in 0..times {
                assert!(bound.move_to().whitespace());
                let token = bound.token().unwrap();
                assert_eq!(token.content, *word);
                let from = n * (word.len() + 1);
                assert_eq!(token.coors, (from, from + word.len() - 1));
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
            let mut bound = Walker::new(format!("{char}{word}"));
            assert!(bound.contains().char(*char));
            let mut bound = Walker::new(format!(r"\\{char}{char}{word}"));
            assert!(bound.contains().char(*char));
            let mut bound = Walker::new(format!(r"\\{char}{word}"));
            assert!(!bound.contains().char(*char));
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
            let mut bound = Walker::new(format!("{target}{word}"));
            assert!(bound.contains().word(&[target]));
            let mut bound = Walker::new(format!(r"\\{target}{target}{word}"));
            assert!(bound.contains().word(&[target]));
            let mut bound = Walker::new(format!(r"\\{target}{word}"));
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
                let mut bound = Walker::new(format!(" \t\n {left}{content}{right}{noise}"));
                let between = bound.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let mut bound = Walker::new(between);
                let between = bound.group().between(left, right).unwrap();
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, noise);
            }
            {
                // Nested shifted groups
                let content = format!("{noise}\\{left}{left}{noise}{right}\\{right}{noise}");
                let mut bound = Walker::new(format!("{left}{content}{right}{noise}"));
                let between = bound.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let mut bound = Walker::new(between);
                bound.until().char(&[*left]);
                let between = bound.group().between(left, right).unwrap();
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, noise);
            }
            {
                // Following groups with spaces between
                let content = format!("{noise}\\{left}{noise}\\{right}{noise}");
                let mut bound = Walker::new(format!(
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
                    Walker::new(format!("{left}{content}{right}{left}{content}{right}"));
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
                let mut bound = Walker::new(format!(" \t\n {border}{noise}{border}"));
                let between = bound.group().closed(border).unwrap();
                assert_eq!(between, noise);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
            }
            {
                let content = format!("\\{border}{noise}\\{border}");
                let mut bound = Walker::new(format!("{border}{content}{border}"));
                let between = bound.group().closed(border).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
            }
            {
                // Following groups without spaces
                let content = format!("\\{border}{noise}\\{border}");
                let mut bound = Walker::new(format!(
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
                let mut bound = Walker::new(format!(
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
        let mut bound = Walker::new(format!("[{inner}]"));
        let between = bound.group().between(&'[', &']').unwrap();
        assert_eq!(between, inner);
        let mut token = bound.token().unwrap();
        assert_eq!(token.content, inner);
        assert_eq!(token.coors, (1, inner.len()));
        let between = token.bound.group().between(&'<', &'>').unwrap();
        assert_eq!(between, noise);
        let nested_token = token.bound.token().unwrap();
        assert_eq!(nested_token.content, noise);
        assert_eq!(nested_token.coors, (2, noise.len() + 1));
    }
}
