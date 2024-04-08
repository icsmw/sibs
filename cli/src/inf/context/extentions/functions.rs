use crate::{
    executors::ExecutorFn,
    inf::{context::E, Context},
};
use std::collections::hash_map::Entry;

#[derive(Debug)]
pub struct Functions<'a> {
    bound: &'a mut Context,
}
impl<'a> Functions<'a> {
    pub fn new(bound: &'a mut Context) -> Self {
        Self { bound }
    }
    pub fn add<'b, T>(&mut self, name: T, func: ExecutorFn) -> Result<(), E>
    where
        T: 'b + ToOwned + ToString,
    {
        if let Entry::Vacant(e) = self.bound.executors.entry(name.to_string()) {
            e.insert(func);
            Ok(())
        } else {
            Err(E::FunctionAlreadyExists(name.to_string()))
        }
    }

    pub fn get(&self, name: &str) -> Option<&ExecutorFn> {
        self.bound.executors.get(name)
    }
}
