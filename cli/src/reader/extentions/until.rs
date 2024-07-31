use crate::reader::{chars, Reader};

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
                self.bound.index(None, self.bound.pos, pos);
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
                    self.bound
                        .index(None, self.bound.pos, pos - (word.len() - 1));
                    self.bound.pos = next_pos;
                    return Some((read, word.to_string()));
                }
            }
        }
        None
    }
}
