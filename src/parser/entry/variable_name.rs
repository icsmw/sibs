#[derive(Debug)]
pub struct VariableName {
    pub name: String,
}

impl VariableName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
