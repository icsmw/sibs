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
    inf::{
        map::{Fragment, Mapping},
        Journal,
    },
    reader::E,
};
use std::{cell::RefCell, collections::HashSet, fmt, fs, path::PathBuf, rc::Rc};
use uuid::Uuid;

pub type MapRef = Rc<RefCell<Map>>;
pub type IdsRef = Rc<RefCell<Ids>>;
type ErrRecord = (usize, Fragment, String);

#[derive(Debug)]
pub struct Sources {
    maps: Maps,
    journal: Journal,
    ids: IdsRef,
    reported: HashSet<Uuid>,
}

impl Sources {
    pub fn new(journal: &Journal) -> Self {
        Self {
            maps: Maps::new(),
            journal: journal.clone(),
            ids: Ids::new(),
            reported: HashSet::new(),
        }
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, PathBuf, MapRef> {
        self.maps.iter()
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
    pub fn report_err<T: Clone>(&mut self, err: &LinkedErr<T>) -> Result<(), E>
    where
        T: std::error::Error + fmt::Display + ToString,
    {
        let Some(token) = err.token.as_ref() else {
            self.journal.report(err.into());
            return Ok(());
        };
        if self.reported.contains(&err.uuid) {
            return Ok(());
        }
        let mut map = self.maps.get(token)?;
        self.journal
            .report((map.report_err(token, err.e.to_string())?, err).into());
        self.reported.insert(err.uuid);
        Ok(())
    }
    #[cfg(test)]
    pub fn report_err_if<T, E: Clone>(
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
