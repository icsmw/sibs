#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Variable {
    pub ident: String,
    pub token: Token,
    pub negation: Option<Token>,
    pub uuid: Uuid,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.negation
                .as_ref()
                .map(|tk| format!("{tk} "))
                .unwrap_or_default(),
            self.token,
        )
    }
}

impl From<Variable> for Node {
    fn from(val: Variable) -> Self {
        Node::Expression(Expression::Variable(val))
    }
}
