use crate::{
    elements::ElementId,
    inf::map::{MapFragment, Mapping},
    reader::Ids,
};
use hashbrown::HashMap;
use std::{cell::RefCell, path::PathBuf, rc::Rc};

pub type MapRestorePoint = (Option<usize>, HashMap<usize, MapFragment>);

#[derive(Debug, Clone)]
pub struct Map {
    pub fragments: HashMap<usize, MapFragment>,
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
    pub fn contains_token(&self, token: &usize) -> bool {
        self.fragments.contains_key(token)
    }
    pub fn pin(&self) -> impl Fn(&mut Map) -> MapRestorePoint {
        let last = self.recent;
        move |map: &mut Map| {
            let to_restore = map.recent;
            if let Some(id) = last {
                map.recent = Some(id);
                (
                    to_restore,
                    map.fragments.extract_if(|k, _| k > &id).collect(),
                )
                // map.fragments.retain(|k, _| k <= &id);
            } else {
                map.recent = None;
                (to_restore, map.fragments.drain().collect())
            }
        }
    }
    pub fn restore(&mut self, point: MapRestorePoint) {
        self.recent = point.0;
        self.fragments.extend(point.1);
    }
    pub fn last(&self) -> Option<(usize, MapFragment)> {
        if let Some(id) = self.recent {
            self.fragments.get(&id).map(|coors| (id, coors.clone()))
        } else {
            None
        }
    }
    pub fn add(&mut self, el: Option<ElementId>, from: usize, len: usize) -> usize {
        let id = self.ids.borrow_mut().get();
        self.recent = Some(id);
        self.fragments.insert(id, MapFragment::new(el, from, len));
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
    fn get_fragments(&self) -> &HashMap<usize, MapFragment> {
        &self.fragments
    }
}
