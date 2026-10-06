use crate::*;
use std::collections::HashMap;

/// Token streams indexed by source identity. Indices are local to each source.
#[derive(Debug)]
pub struct TokenStore {
    root: Uuid,
    sources: HashMap<Uuid, Tokens>,
}

impl TokenStore {
    pub fn new(root: Uuid, tokens: Tokens) -> Self {
        Self {
            root,
            sources: HashMap::from([(root, tokens)]),
        }
    }

    pub fn root(&self) -> &Uuid {
        &self.root
    }

    pub fn get(&self, source: &Uuid) -> Option<&Tokens> {
        self.sources.get(source)
    }

    pub fn insert(&mut self, source: Uuid, tokens: Tokens) {
        self.sources.insert(source, tokens);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Tokens)> {
        self.sources.iter()
    }
}
