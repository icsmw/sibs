use uuid::Uuid;

use crate::{inf::map::Mapping, reader::Ids};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

#[derive(Debug, Clone)]
pub struct Map {
    //              <id,    (from,  len  )>
    pub fragments: HashMap<usize, (usize, usize)>,
    pub content: String,
    pub filename: PathBuf,
    recent: Option<usize>,
    ids: Rc<RefCell<Ids>>,
}

impl Map {
    pub fn new(ids: Rc<RefCell<Ids>>, filename: &PathBuf, content: &str) -> Self {
        Self {
            fragments: HashMap::new(),
            content: content.to_owned(),
            filename: filename.to_owned(),
            recent: None,
            ids,
        }
    }
    pub fn unbound(content: &str) -> Self {
        Self {
            fragments: HashMap::new(),
            content: content.to_owned(),
            filename: PathBuf::from(Uuid::new_v4().to_string()),
            recent: None,
            ids: Ids::new(),
        }
    }
    pub fn contains_token(&self, token: &usize) -> bool {
        self.fragments.contains_key(token)
    }
    pub fn pin(&self) -> impl Fn(&mut Map) {
        let last = self.recent;
        move |map: &mut Map| {
            if let Some(id) = last {
                map.recent = Some(id);
                map.fragments.retain(|k, _| k <= &id);
            } else {
                map.recent = None;
                map.fragments.clear();
            }
        }
    }
    pub fn last(&self) -> Option<(usize, (usize, usize))> {
        if let Some(id) = self.recent {
            self.fragments.get(&id).map(|coors| (id, *coors))
        } else {
            None
        }
    }
    pub fn add(&mut self, from: usize, len: usize) -> usize {
        let id = self.ids.borrow_mut().get();
        self.recent = Some(id);
        self.fragments.insert(id, (from, len));
        id
    }
}

impl Mapping for Map {
    fn get_filename(&self) -> &PathBuf {
        &self.filename
    }
    fn get_content(&self) -> &str {
        &self.content
    }
    fn get_fragments(&self) -> &HashMap<usize, (usize, usize)> {
        &self.fragments
    }
}
