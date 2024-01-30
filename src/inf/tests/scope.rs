use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    pub variables: HashMap<String, String>,
}
