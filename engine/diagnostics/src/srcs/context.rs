use uuid::Uuid;

/// One branch of source loading. Sibling imports keep independent ancestry.
#[derive(Debug, Clone)]
pub struct CodeSourceContext {
    pub source: Uuid,
    pub(super) ancestry: Vec<Uuid>,
}

impl CodeSourceContext {
    /// Start a branch at an already registered source, including an inline source.
    pub fn new(source: Uuid) -> Self {
        Self {
            source,
            ancestry: vec![source],
        }
    }
}
