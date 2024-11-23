#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableTypeDeclaration {
    pub types: Vec<Node>,
    pub token: Token,
}

impl fmt::Display for VariableTypeDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.token,
            self.types
                .iter()
                .map(|ty| ty.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::VerticalBar))
        )
    }
}
