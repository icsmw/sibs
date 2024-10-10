use crate::elements::{function::Function, TokenGetter};
use std::fmt;

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

impl TokenGetter for Function {
    fn token(&self) -> usize {
        self.token
    }
}
