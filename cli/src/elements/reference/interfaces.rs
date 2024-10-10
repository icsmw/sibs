use crate::elements::{Reference, TokenGetter};
use std::{
    cmp::{Eq, PartialEq},
    fmt,
};

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Reference {}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            ":{}{}",
            self.path.join(":"),
            if self.inputs.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    self.inputs
                        .iter()
                        .map(|input| input.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        )
    }
}

impl TokenGetter for Reference {
    fn token(&self) -> usize {
        self.token
    }
}
