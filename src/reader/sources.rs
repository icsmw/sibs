use crate::{
    error::LinkedErr,
    inf::{AnyValue, Trace},
    reader::{ids::Ids, map::Map, maps::Maps, E},
};
use std::{cell::RefCell, fmt, path::PathBuf, rc::Rc};

pub type MapRef = Rc<RefCell<Map>>;
pub type IdsRef = Rc<RefCell<Ids>>;

#[derive(Debug)]
pub struct Sources {
    maps: Maps,
    ids: IdsRef,
    trace: Trace,
    dummy: usize,
}

impl Sources {
    pub fn new() -> Self {
        Self {
            maps: Maps::new(),
            ids: Ids::new(),
            trace: Trace::new(None),
            dummy: 0,
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
            .insert(&PathBuf::from(self.dummy.to_string()), map.clone())?;
        self.dummy += 1;
        Ok(map)
    }
    pub fn report_error<T>(&mut self, err: &LinkedErr<T>) -> Result<(), E>
    where
        T: std::error::Error + fmt::Display + ToString,
    {
        if let Some(token) = err.token.as_ref() {
            self.trace.add_report(&self.maps, token, err)
        } else {
            Ok(())
        }
    }
    pub fn set_map_cursor(&self, token: &usize) -> Result<(), E> {
        self.maps.get(token)?.set_cursor(*token);
        Ok(())
    }
    pub fn set_trace_value(&mut self, token: &usize, value: &Option<AnyValue>) {
        self.trace.add(token, value);
    }
}
