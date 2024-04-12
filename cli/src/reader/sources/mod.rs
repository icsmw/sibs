mod extentions;
mod ids;
mod map;
mod maps;

pub use extentions::*;
pub use ids::*;
pub use map::*;
pub use maps::*;

use crate::{
    error::LinkedErr,
    inf::map::{Fragment, Mapping},
    reader::E,
};
use std::{cell::RefCell, collections::HashMap, fmt, fs, path::PathBuf, rc::Rc};
use uuid::Uuid;

pub type MapRef = Rc<RefCell<Map>>;
pub type IdsRef = Rc<RefCell<Ids>>;
type ErrRecord = (usize, Fragment, String);

#[derive(Debug)]
pub struct Sources {
    maps: Maps,
    ids: IdsRef,
    errs: HashMap<Uuid, ErrRecord>,
}

impl Sources {
    pub fn new() -> Self {
        Self {
            maps: Maps::new(),
            ids: Ids::new(),
            errs: HashMap::new(),
        }
    }
    pub fn reader(&mut self) -> ReaderGetter<'_> {
        ReaderGetter::new(self)
    }
    pub fn add_from_file(&mut self, filename: &PathBuf) -> Result<MapRef, E> {
        let map = Rc::new(RefCell::new(Map::new(
            self.ids.clone(),
            filename,
            &fs::read_to_string(filename)?,
        )));
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
    pub fn report_err<T>(&mut self, err: &LinkedErr<T>) -> Result<(), E>
    where
        T: std::error::Error + fmt::Display + ToString,
    {
        let token = err.token.as_ref().ok_or(E::LinkedErrorWithoutToken)?;
        if self.errs.contains_key(&err.uuid) {
            return Ok(());
        }
        let mut map = self.maps.get(token)?;
        self.errs.insert(
            err.uuid,
            (
                *token,
                map.get_fragment(token)?,
                map.report_err(token, err.e.to_string())?,
            ),
        );
        Ok(())
    }
    #[cfg(test)]
    pub fn report_err_if<T, E>(
        &mut self,
        result: Result<T, LinkedErr<E>>,
    ) -> Result<T, LinkedErr<E>>
    where
        E: std::error::Error + fmt::Display + ToString,
    {
        if let Err(err) = result.as_ref() {
            self.report_err(err).expect("Error report created");
        }
        result
    }
}
