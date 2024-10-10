use crate::elements::{Closure, TokenGetter};
use std::fmt;

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}) {}",
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.block,
        )
    }
}

impl TokenGetter for Closure {
    fn token(&self) -> usize {
        self.token
    }
}
