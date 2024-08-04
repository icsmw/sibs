use crate::reader::Reader;
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
                self.bound.index(None, self.bound.pos, pos);
                self.bound.pos += pos + 1;
                Some(char)
            } else {
                None
            };
        }
        None
    }
    pub fn any(&mut self) {
        if self.bound.done() {
            return;
        }
        let content = &self.bound.content[self.bound.pos..];
        for char in content.chars() {
            if char.is_whitespace() {
                self.bound.pos += 1;
            } else {
                break;
            }
        }
    }
    pub fn word(&mut self, words: &[&str]) -> Option<String> {
        if self.bound.done() {
            return None;
        }
        let content = &self.bound.content[self.bound.pos..].trim();
        let mut found: Option<String> = None;
        for word in words.iter() {
            if content.starts_with(word) {
                if let Some(char) = content.chars().nth(word.len()) {
                    if char.is_alphabetic() || char.is_alphanumeric() {
                        continue;
                    }
                }
                found = Some(word.to_string());
                break;
            }
        }
        if let Some(found) = found {
            let from = self.bound.pos;
            self.any();
            self.bound.pos += found.len();
            self.bound.index(None, from, self.bound.pos - from);
            Some(found)
        } else {
            None
        }
    }
    pub fn expression(&mut self, words: &[&str]) -> Option<String> {
        if self.bound.done() {
            return None;
        }
        let content = &self.bound.content[self.bound.pos..].trim();
        let mut found: Option<String> = None;
        for word in words.iter() {
            if content.starts_with(word) {
                found = Some(word.to_string());
                break;
            }
        }
        if let Some(found) = found {
            let from = self.bound.pos;
            self.any();
            self.bound.pos += found.len();
            self.bound.index(None, from, self.bound.pos - from);
            Some(found)
        } else {
            None
        }
    }
    pub fn none_numeric(&mut self) -> Option<String> {
        if self.bound.done() {
            return None;
        }
        let mut str: String = String::new();
        let content = &self.bound.content[self.bound.pos..];
        let mut negative = false;
        let mut first: Option<usize> = None;
        for (pos, char) in content.chars().enumerate() {
            if char.is_whitespace() && first.is_none() {
                continue;
            }
            if char == '-' && !negative && first.is_none() {
                negative = true;
                str.push(char);
                continue;
            } else if char.is_numeric() {
                first = Some(pos);
                str.push(char);
                continue;
            }
            if !str.is_empty() && str != "-" {
                self.bound.index(None, self.bound.pos, pos);
                self.bound.pos += pos;
                return Some(str);
            } else {
                return None;
            }
        }
        if !str.is_empty() && str != "-" {
            let last = content.len();
            self.bound.index(None, self.bound.pos, last);
            self.bound.pos += last;
            Some(str)
        } else {
            None
        }
    }
    pub fn next(&mut self) -> bool {
        if self.bound.pos < self.bound.content.len() {
            self.bound.pos += 1;
            true
        } else {
            false
        }
    }
    #[cfg(test)]
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
        self.bound.index(None, self.bound.pos, pos - self.bound.pos);
        self.bound.pos = pos;
        rest
    }
}
