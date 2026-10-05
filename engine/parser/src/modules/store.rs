use asttree::ModuleBody;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Default)]
pub(crate) struct ModuleStore {
    modules: HashMap<PathBuf, Arc<ModuleBody>>,
}

impl ModuleStore {
    pub(crate) fn get(&self, path: &Path) -> Option<Arc<ModuleBody>> {
        self.modules.get(path).cloned()
    }

    pub(crate) fn insert(&mut self, path: PathBuf, body: Arc<ModuleBody>) {
        self.modules.insert(path, body);
    }
}
