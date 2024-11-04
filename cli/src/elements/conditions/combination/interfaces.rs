use crate::{
    elements::{conditions::Cmb, Combination, TokenGetter},
    reader::words,
};
use std::fmt;

impl fmt::Display for Cmb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => words::AND,
                Self::Or => words::OR,
            }
        )
    }
}

impl fmt::Display for Combination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cmb)
    }
}

impl TokenGetter for Combination {
    fn token(&self) -> usize {
        self.token
    }
}
