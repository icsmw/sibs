use crate::elements::{TokenGetter, Values};
use std::fmt;

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({})",
            self.elements
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl TokenGetter for Values {
    fn token(&self) -> usize {
        self.token
    }
}
