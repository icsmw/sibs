use crate::elements::{IfSubsequence, TokenGetter};
use std::fmt;

impl fmt::Display for IfSubsequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.subsequence
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl TokenGetter for IfSubsequence {
    fn token(&self) -> usize {
        self.token
    }
}
