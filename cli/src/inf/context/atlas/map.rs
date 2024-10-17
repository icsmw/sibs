use crate::{
    inf::map::{MapFragment, Mapping},
    reader::Map as ReaderMap,
};
use hashbrown::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Map {
    pub fragments: HashMap<usize, MapFragment>,
    pub content: String,
    filename: PathBuf,
    cursor: Option<usize>,
}

impl Map {
    pub fn set_cursor(&mut self, token: usize) {
        self.cursor = Some(token);
    }
}

impl Mapping for Map {
    fn get_filename(&self) -> &PathBuf {
        &self.filename
    }
    fn get_fragments(&self) -> &HashMap<usize, MapFragment> {
        &self.fragments
    }
    fn get_content(&self) -> &str {
        &self.content
    }
}

impl From<ReaderMap> for Map {
    fn from(map: ReaderMap) -> Self {
        Self {
            content: map.content,
            fragments: map.fragments,
            filename: map.filename,
            cursor: None,
        }
    }
}
