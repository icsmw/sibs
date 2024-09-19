mod error;

pub use error::E;
use std::{collections::HashMap, sync::Arc};

use crate::reader::words;

#[derive(Debug)]
pub struct Store<T> {
    executors: HashMap<String, Arc<T>>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            executors: HashMap::new(),
        }
    }
    pub fn insert<S: AsRef<str>>(&mut self, name: S, executor: T) -> Result<(), E> {
        let name = name.as_ref().to_string();
        for part in name.split("::") {
            if words::is_reserved(part) {
                return Err(E::ReservedName(part.to_owned()));
            }
        }
        if self.executors.contains_key(&name) {
            return Err(E::ItemAlreadyExists(name));
        }
        self.executors.insert(name, Arc::new(executor));
        Ok(())
    }
    pub fn get(&self, name: &str) -> Option<Arc<T>> {
        self.executors.get(name).cloned()
    }
    pub fn all(&self) -> &HashMap<String, Arc<T>> {
        &self.executors
    }
}
