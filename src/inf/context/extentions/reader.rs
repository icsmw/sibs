use crate::{
    inf::Context,
    reader::{Reader, E},
};
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct ReaderGetter<'a> {
    bound: &'a mut Context,
}
impl<'a> ReaderGetter<'a> {
    pub fn new(bound: &'a mut Context) -> Self {
        Self { bound }
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn from_file(&mut self, filename: &PathBuf) -> Result<Reader, E> {
        Ok(Reader::new(
            self.bound
                .sources
                .add(filename, &fs::read_to_string(filename)?)?,
        ))
    }
    #[cfg(test)]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_str(&mut self, content: &str) -> Reader {
        Reader::new(self.bound.sources.add_from_str(content))
    }
}
