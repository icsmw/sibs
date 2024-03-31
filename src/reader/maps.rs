use crate::reader::{map::Map, E};
use std::{
    cell::{RefCell, RefMut},
    collections::{hash_map, HashMap},
    path::PathBuf,
    rc::Rc,
};

pub type MapRef = Rc<RefCell<Map>>;

#[derive(Debug)]
pub struct Maps {
    maps: HashMap<PathBuf, MapRef>,
}
impl Maps {
    pub fn new() -> Self {
        Self {
            maps: HashMap::new(),
        }
    }
    pub fn insert(&mut self, filename: &PathBuf, map: MapRef) -> Result<MapRef, E> {
        if self.maps.contains_key(filename) {
            Err(E::FileAlreadyHasMap(filename.to_owned()))?;
        }
        self.maps.insert(filename.to_owned(), map.clone());
        Ok(map)
    }
    pub fn get(&self, token: &usize) -> Result<RefMut<'_, Map>, E> {
        for (_, map) in self.maps.iter() {
            if map.borrow().contains_token(token) {
                return Ok(map.borrow_mut());
            }
        }
        Err(E::FailToFindToken(*token))
    }
}
