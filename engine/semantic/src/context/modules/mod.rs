use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
enum ModuleState {
    Initialized,
    Finalized,
}

/// Completed analysis stages of module bodies in one semantic context.
#[derive(Debug, Default)]
pub struct Modules {
    modules: HashMap<Uuid, ModuleState>,
}

impl Modules {
    pub fn is_initialized(&self, source: &Uuid) -> bool {
        self.modules.contains_key(source)
    }
    pub fn is_finalized(&self, source: &Uuid) -> bool {
        matches!(self.modules.get(source), Some(ModuleState::Finalized))
    }
    /// Record successful initialization without resetting a finalized module.
    pub fn initialized(&mut self, source: Uuid) {
        self.modules
            .entry(source)
            .or_insert(ModuleState::Initialized);
    }
    /// Record successful finalization, which also implies initialization.
    pub fn finalized(&mut self, source: Uuid) {
        self.modules.insert(source, ModuleState::Finalized);
    }
}

#[cfg(test)]
mod tests;
