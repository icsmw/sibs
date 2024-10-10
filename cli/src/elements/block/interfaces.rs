use crate::elements::{Block, TokenGetter};
use std::fmt;

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\n{}{}}}",
            self.elements
                .iter()
                .map(|el| format!("{el};",))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.elements.is_empty() { "" } else { "\n" }
        )
    }
}

impl TokenGetter for Block {
    fn token(&self) -> usize {
        self.token
    }
}
