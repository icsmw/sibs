use crate::reader::Reader;

#[derive(Debug)]
pub struct Next<'a> {
    bound: &'a Reader,
}
impl<'a> Next<'a> {
    pub fn new(bound: &'a Reader) -> Self {
        Self { bound }
    }
    pub fn is_word(&self, words: &[&str]) -> bool {
        if self.bound.done() {
            return false;
        }
        let trimmed = self.bound.content[self.bound.pos..].trim();
        for word in words.iter() {
            if trimmed.starts_with(word) {
                return true;
            }
        }
        false
    }
    #[cfg(test)]
    pub fn char(&self) -> Option<char> {
        if self.bound.done() {
            return None;
        }
        self.bound.content[self.bound.pos..].chars().next()
    }
}
