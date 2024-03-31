use crate::{
    error::LinkedErr,
    inf::{AnyValue, Trace},
    reader::{ids::Ids, map::Map, E},
};
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    fmt,
    path::PathBuf,
    rc::Rc,
};

pub type MapRef = Rc<RefCell<Map>>;
pub type IdsRef = Rc<RefCell<Ids>>;

#[derive(Debug)]
pub struct Sources {
    maps: HashMap<PathBuf, MapRef>,
    ids: IdsRef,
    trace: Trace,
    #[cfg(test)]
    dummy: usize,
}

impl Sources {
    fn get_map_by_token(&self, token: &usize) -> Result<RefMut<'_, Map>, E> {
        for (_, map) in self.maps.iter() {
            if map.borrow().contains_token(token) {
                return Ok(map.borrow_mut());
            }
        }
        Err(E::FailToFindToken(*token))
    }
    pub fn new() -> Self {
        Self {
            maps: HashMap::new(),
            ids: Ids::new(),
            trace: Trace::new(None),
            #[cfg(test)]
            dummy: 0,
        }
    }
    pub fn add(&mut self, filename: &PathBuf, content: &str) -> Result<MapRef, E> {
        if self.maps.contains_key(filename) {
            Err(E::FileAlreadyHasMap(filename.to_owned()))?;
        }
        let map = Rc::new(RefCell::new(Map::new(self.ids.clone(), filename, content)));
        self.maps.insert(filename.to_owned(), map.clone());
        Ok(map)
    }
    #[cfg(test)]
    pub fn add_from_str(&mut self, content: &str) -> MapRef {
        let map = Rc::new(RefCell::new(Map::new(
            self.ids.clone(),
            &PathBuf::new(),
            content,
        )));
        self.maps
            .insert(PathBuf::from(self.dummy.to_string()), map.clone());
        self.dummy += 1;
        map
    }
    pub fn gen_report_from_err<T>(&self, err: &LinkedErr<T>) -> Result<(), E>
    where
        T: fmt::Display + ToString,
    {
        if let Some(token) = err.token.as_ref() {
            self.get_map_by_token(token)?
                .get_err_report(token, err.e.to_string())?;
        }
        Ok(())
    }
    pub fn post_reports(&self) {
        self.maps
            .iter()
            .for_each(|(_, map)| map.borrow().post_reports());
    }
    pub fn report_error<T>(&mut self, token: &usize, err: &T) -> Result<(), E>
    where
        T: std::error::Error + fmt::Display + ToString,
    {
        println!(">>>>>>>>>>>>>>>>>>>> REPORT ERROR ON: {token}");
        for (token, value) in self.trace.iter() {
            println!(
                "{}",
                self.get_map_by_token(token)?.get_report(token, value,)?
            );
        }
        println!(
            "{}",
            self.get_map_by_token(token)?.get_err_report(token, err)?
        );
        Ok(())
    }
    pub fn set_map_cursor(&self, token: &usize) -> Result<(), E> {
        self.get_map_by_token(token)?.set_cursor(*token);
        Ok(())
    }
    pub fn set_trace_value(&mut self, token: &usize, value: &Option<AnyValue>) {
        self.trace.add(token, value);
    }
}
