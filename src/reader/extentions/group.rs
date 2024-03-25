use crate::reader::{chars, Reader};

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
}
