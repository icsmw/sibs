use crate::parser::entry::{Values, VariableType};

#[derive(Debug)]
pub enum Declaration {
    Typed(VariableType),
    Values(Values),
}
#[derive(Debug)]
pub struct VariableDeclaration {
    pub declaration: Declaration,
}

impl VariableDeclaration {
    pub fn typed(typed: VariableType) -> Self {
        Self {
            declaration: Declaration::Typed(typed),
        }
    }
    pub fn values(values: Values) -> Self {
        Self {
            declaration: Declaration::Values(values),
        }
    }
}
