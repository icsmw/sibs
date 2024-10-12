use crate::elements::{If, TokenGetter};
use std::fmt;

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.threads
                .iter()
                .map(|el| el.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl TokenGetter for If {
    fn token(&self) -> usize {
        self.token
    }
}
