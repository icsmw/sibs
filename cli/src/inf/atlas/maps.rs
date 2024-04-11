use crate::inf::{atlas, atlas::E, map::Map};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Maps<'a> {
    maps: &'a mut HashMap<PathBuf, atlas::Map>,
}
impl<'a> Maps<'a> {
    pub fn new(maps: &'a mut HashMap<PathBuf, atlas::Map>) -> Self {
        Self { maps }
    }
    pub fn get(&mut self, token: &usize) -> Result<&mut atlas::Map, E> {
        for (_, map) in self.maps.iter_mut() {
            if map.contains(token) {
                return Ok(map);
            }
        }
        Err(E::FailToFindToken(*token))
    }
}
