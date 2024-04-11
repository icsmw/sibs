mod ids;
mod map;
mod maps;

use crate::{
    error::LinkedErr,
    inf::{AnyValue, Trace},
    reader::E,
};
pub use ids::*;
pub use map::*;
pub use maps::*;
use std::{cell::RefCell, fmt, path::PathBuf, rc::Rc};
use uuid::Uuid;

pub type MapRef = Rc<RefCell<Map>>;
pub type IdsRef = Rc<RefCell<Ids>>;

#[derive(Debug)]
pub struct Sources {
    maps: Maps,
    ids: IdsRef,
    trace: Trace,
}

impl Sources {
    pub fn new() -> Self {
        Self {
            maps: Maps::new(),
            ids: Ids::new(),
            trace: Trace::new(None),
        }
    }
    pub fn add(&mut self, filename: &PathBuf, content: &str) -> Result<MapRef, E> {
        let map = Rc::new(RefCell::new(Map::new(self.ids.clone(), filename, content)));
        self.maps.insert(filename, map.clone())
    }
    pub fn add_from_str(&mut self, content: &str) -> Result<MapRef, E> {
        let map = Rc::new(RefCell::new(Map::new(
            self.ids.clone(),
            &PathBuf::new(),
            content,
        )));
        self.maps
            .insert(&PathBuf::from(Uuid::new_v4().to_string()), map.clone())?;
        Ok(map)
    }
    pub fn report_error<T>(&mut self, err: &LinkedErr<T>) -> Result<(), E>
    where
        T: std::error::Error + fmt::Display + ToString,
    {
        Ok(if let Some(token) = err.token.as_ref() {
            self.trace.add_report(&self.maps, token, err)?
        })
    }
    pub fn set_map_cursor(&self, token: &usize) -> Result<(), E> {
        self.maps.get(token)?.set_cursor(*token);
        Ok(())
    }
    pub fn set_trace_value(&mut self, token: &usize, value: &Option<AnyValue>) {
        self.trace.add(token, value);
    }
}
