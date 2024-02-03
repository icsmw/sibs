use crate::reader::entry::{ValueString, VariableName};
use proptest::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub declarations: HashMap<String, Option<String>>,
    pub assignation: HashMap<String, Option<String>>,
}

impl Scope {
    pub fn get_rnd_declaration_name(&self) -> BoxedStrategy<String> {
        prop::sample::select(self.declarations.keys().cloned().collect::<Vec<String>>()).boxed()
    }
    pub fn add_declaration(&mut self, name: String) {
        self.declarations.insert(name, None);
    }
    pub fn assign_declaration(&mut self, name: String, value: String) {
        self.declarations
            .entry(name)
            .and_modify(|v| {
                let _ = v.insert(value.clone());
            })
            .or_insert(Some(value));
    }
    pub fn add_assignation(&mut self, name: String) {
        self.assignation.insert(name, None);
    }
    pub fn assign_assignation(&mut self, name: String, value: String) {
        self.assignation
            .entry(name)
            .and_modify(|v| {
                let _ = v.insert(value.clone());
            })
            .or_insert(Some(value));
    }
}

pub type SharedScope = Arc<RwLock<Scope>>;
