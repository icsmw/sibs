use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub variables: HashMap<String, String>,
}

pub type SharedScope = Arc<RwLock<Scope>>;
