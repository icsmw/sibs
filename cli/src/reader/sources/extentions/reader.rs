use crate::reader::{Reader, Sources, E};
use std::path::PathBuf;

#[derive(Debug)]
pub struct ReaderGetter<'a> {
    bound: &'a mut Sources,
}
impl<'a> ReaderGetter<'a> {
    pub fn new(bound: &'a mut Sources) -> Self {
        Self { bound }
    }

    pub fn bound(&mut self, filename: &PathBuf) -> Result<Reader, E> {
        Reader::bound(&mut self.bound, filename)
    }

    pub fn unbound(&self, content: &str) -> Result<Reader, E> {
        Reader::unbound(content)
    }
}
