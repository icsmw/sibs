use crate::*;

/// A parsed module source shared by all declarations that import it.
#[derive(Debug)]
pub struct ModuleBody {
    pub source: Uuid,
    pub nodes: Vec<LinkedNode>,
}
