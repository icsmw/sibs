mod error;

pub use error::E;
use std::{collections::HashMap, sync::Arc};

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
    pub fn insert<'b, N>(&mut self, name: N, executor: T) -> Result<(), E>
    where
        N: 'b + ToOwned + ToString,
    {
        let name = name.to_string();
        if self.executors.contains_key(&name) {
            return Err(E::ItemAlreadyExists(name));
        }
        self.executors.insert(name, Arc::new(executor));
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Arc<T>> {
        self.executors.get(name).cloned()
    }
}
