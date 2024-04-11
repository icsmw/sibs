use crate::inf::{atlas::E, map};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Map {
    //              <id,    (from,  len  )>
    pub fragments: HashMap<usize, (usize, usize)>,
    pub reports: Vec<String>,
    pub content: String,
    filename: PathBuf,
    cursor: Option<usize>,
}

impl Map {
    pub fn new(filename: &PathBuf, content: &str) -> Self {
        Self {
            fragments: HashMap::new(),
            reports: vec![],
            content: content.to_owned(),
            filename: filename.to_owned(),
            cursor: None,
        }
    }
    pub fn set_cursor(&mut self, token: usize) {
        self.cursor = Some(token);
    }
}

impl map::Map for Map {
    fn get_fragments(&self) -> &HashMap<usize, (usize, usize)> {
        &self.fragments
    }
    fn get_content(&self) -> &str {
        &self.content
    }
}
