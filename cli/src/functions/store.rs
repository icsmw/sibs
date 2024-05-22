use crate::functions::{ExecutorFn, E};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug)]
pub struct Store {
    executors: HashMap<String, Arc<ExecutorFn>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            executors: HashMap::new(),
        }
    }
    pub fn insert<'b, T>(&mut self, name: T, executor: ExecutorFn) -> Result<(), E>
    where
        T: 'b + ToOwned + ToString,
    {
        let name = name.to_string();
        if self.executors.contains_key(&name) {
            return Err(E::FunctionExists(name));
        }
        self.executors.insert(name, Arc::new(executor));
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Arc<ExecutorFn>> {
        self.executors.get(name).cloned()
    }
}
