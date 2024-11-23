#[cfg(feature = "proptests")]
mod proptests;

use crate::Node;
use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableVariants {
    pub variants: Vec<Node>,
    pub token: Token,
}

impl fmt::Display for VariableVariants {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.token,
            self.variants
                .iter()
                .map(|ty| ty.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::VerticalBar))
        )
    }
}
