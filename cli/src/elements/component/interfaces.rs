use crate::elements::{Component, TokenGetter};
use std::fmt;

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#({}{}){}",
            self.name,
            self.cwd
                .as_ref()
                .map(|cwd| format!(": {}", cwd.display()))
                .unwrap_or_default(),
            self.elements
                .iter()
                .map(|el| format!("{el};"))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl TokenGetter for Component {
    fn token(&self) -> usize {
        self.token
    }
}
